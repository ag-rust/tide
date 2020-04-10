use async_std::task;

fn main() -> Result<(), std::io::Error> {
    femme::start(log::LevelFilter::Error).unwrap();
    task::block_on(async {
        let mut app = tide::new();
        app.at("/").get(|_| async move { "Hello, world!" });
        app.at("/src").serve_dir("src/")?;
        app.listen("127.0.0.1:8080").await?;
        Ok(())
    })
}
