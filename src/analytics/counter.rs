use std::sync::OnceLock;
use tiktoken_rs::cl100k_base;

fn bpe() -> &'static tiktoken_rs::CoreBPE {
    static BPE: OnceLock<tiktoken_rs::CoreBPE> = OnceLock::new();
    BPE.get_or_init(|| cl100k_base().expect("failed to load cl100k_base tokenizer"))
}

pub fn count_tokens(text: &str) -> usize {
    bpe().encode_ordinary(text).len()
}

#[allow(dead_code)]
pub fn estimate_savings(original: &str, filtered: &str) -> (usize, usize, f64) {
    let orig = count_tokens(original);
    let filt = count_tokens(filtered);
    let pct = if orig > 0 {
        ((orig - filt) as f64 / orig as f64) * 100.0
    } else {
        0.0
    };
    (orig, filt, pct)
}
