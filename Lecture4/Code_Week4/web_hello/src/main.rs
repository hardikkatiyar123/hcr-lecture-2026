use hotaru::prelude::*;
use hotaru::http::*;

#[tokio::main]
async fn main() {
    APP.clone().run().await;
}

LServer!(
    APP = Server::new()
        .binding("127.0.0.1:3003")
        .single_protocol(ProtocolBuilder::new(HTTP::server(HttpSafety::default())))
        .build()
);

endpoint!{
    APP.url("/"),

    /// Hello world function
    pub hello_world <HTTP> {
        text_response("Hello, world!")
    }
}
