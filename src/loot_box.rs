use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

pub struct LootBox {
    msgs: Vec<String>,
    rng: ThreadRng,
}

impl LootBox {
    pub fn new(msgs: Vec<String>) -> Self {
        let rng = thread_rng();
        Self { msgs, rng }
    }

    pub fn roll(&mut self) -> String {
        self.msgs
            .choose(&mut self.rng)
            .unwrap_or(&String::new())
            .to_string()
    }
}
