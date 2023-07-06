use rand::{rngs::SmallRng, seq::SliceRandom, thread_rng, SeedableRng};
use std::ops::Deref;

pub static EMOJIS: &[&str] = &[
    "🚀", "🐛", "🌋", "💓", "🤣", "👋", "🥛", "💀", "🦀", "🐰", "🐶", "🦧", "🦜", "🗿", "🤖", "💩",
];

pub fn get_random_emoji<'a>() -> Option<&'a str> {
    let small_rng = &mut SmallRng::from_rng(&mut thread_rng()).ok()?;

    EMOJIS.choose(small_rng).map(|e| e.deref())
}
