use tide::{Request, prelude::*};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(hello_world);
    println!("Listening on 80");
    app.listen("0.0.0.0:80").await?;
    Ok(())
}

async fn hello_world(_: Request<()>) -> tide::Result {
    Ok("Hello, oh maen!".into())
}
