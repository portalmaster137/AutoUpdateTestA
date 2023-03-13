use tide::{Request, prelude::*};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(hello_world);
    app.at("/secret").get(secret);
    println!("Listening on 80");
    app.listen("0.0.0.0:80").await?;
    Ok(())
}

async fn hello_world(_: Request<()>) -> tide::Result {
    Ok("Hello, oh penis!".into())
}

async fn secret(_: Request<()>) -> tide::Result {
    Ok("Secret!".into())
}
