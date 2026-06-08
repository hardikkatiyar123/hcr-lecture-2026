use hotaru::http::*;
use hotaru::prelude::*;

#[tokio::main]
async fn main() {
    APP.clone().run().await;
}

// Local server: listens on 127.0.0.1:3003, speaks plain HTTP.
LServer!(
    APP = Server::new()
        .binding("127.0.0.1:3003")
        .single_protocol(ProtocolBuilder::new(HTTP::server(HttpSafety::default())))
        .build()
);

// Client that talks to timeapi.io over HTTPS (port 443).
//
// `LClient!` hardcodes the static's type as `SClient`, which defaults to
// `Lazy<Arc<Client<TcpTransport>>>`. For HTTPS we need `TlsTransport`, so we
// declare the static directly to pin TS = TlsTransport.
pub static TIME_CLIENT: Lazy<Arc<Client<TlsTransport>>> = Lazy::new(|| {
    Client::<TlsTransport>::new()
        .target(TlsOutboundTarget::new(
            "timeapi.io",
            443,
            TlsClientConfig::default(),
        ))
        .single_protocol(ProtocolBuilder::new(HTTPS::client(HttpSafety::default())))
        .build()
});

// Outpoint: one named outgoing call on TIME_CLIENT.
// The body adjusts the outbound request, then `send;` performs it;
// afterwards `req` holds the response.
outpoint! {
    TIME_CLIENT.url("/api/v1/time/current/utc"),

    fetch_utc_time <HTTPS> {
        send;
        Ok(req)
    }
}

// Server endpoint at /. Fires the outpoint, then forwards the JSON
// the upstream returned. Upstream and lookup errors become plain
// text responses with 502 / 500 status codes.
endpoint! {
    APP.url("/"),

    pub index <HTTP> {
        let mut outbound = request_templates::get_request("/api/v1/time/current/utc");
        // The Host header should be inferred from TIME_CLIENT's target
        // hostname; the framework patch to make this automatic is in
        // progress. Until it lands, set Host explicitly.
        outbound.meta.set_host(Some("timeapi.io".to_string()));

        match run!(TIME_CLIENT<HTTPS>::fetch_utc_time, outbound).await {
            Ok(Ok(resp)) => {
                let body_bytes: Vec<u8> = match resp.body {
                    HttpBody::Text(s)             => s.into_bytes(),
                    HttpBody::Binary(b)           => b,
                    HttpBody::Buffer { data, .. } => data,
                    _                             => Vec::new(),
                };
                let body_str = String::from_utf8_lossy(&body_bytes).to_string();
                text_response(body_str)
            }
            Ok(Err(e)) => response_templates::normal_response(
                502u16, format!("upstream error: {e}"),
            ),
            Err(e) => response_templates::normal_response(
                500u16, format!("lookup error: {e}"),
            ),
        }
    }
}

#[allow(dead_code)]
mod resource;
