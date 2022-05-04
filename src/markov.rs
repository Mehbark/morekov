use markov::Chain;

pub fn feed_post(chain: &mut Chain<String>, post: &str) {
    chain.feed_str(post);
    chain
        .save("saved.yaml")
        .expect("oh crap we failed to save this is bad");
}

pub fn ready_chain() -> Chain<String> {
    Chain::load("saved.yaml").unwrap_or_else(|_| Chain::new())
}
