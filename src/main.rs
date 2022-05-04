use color_eyre::Result;

mod bot;
mod markov;
mod parse_mention;

use bot::Bot;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut cool = Bot::try_load()?;
    cool.chain.feed_file("shakespeare.txt")?;

    loop {
        cool.handle_notifs().await?;
        cool.post_generated().await?;
        tokio::time::sleep(std::time::Duration::from_secs(60 * 30)).await;
    }
}
