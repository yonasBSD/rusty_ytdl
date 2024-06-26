#[tokio::main]
async fn main() {
    use rusty_ytdl::search::YouTube;

    let youtube = YouTube::new().unwrap();

    let res = youtube.suggestion("i know ", None).await;

    println!("{res:#?}");
}
