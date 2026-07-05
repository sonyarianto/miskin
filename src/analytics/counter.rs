use tiktoken_rs::cl100k_base;

pub fn count_tokens(text: &str) -> usize {
    if let Ok(bpe) = cl100k_base() {
        bpe.encode_ordinary(text).len()
    } else {
        text.split_whitespace().count() * 100 / 75
    }
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
