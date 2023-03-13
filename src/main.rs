use tide::{Request, prelude::*};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(hello_world);
    println!("Listening on 8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn hello_world(_: Request<()>) -> tide::Result {
    Ok("Hello, penises!".into())
}
