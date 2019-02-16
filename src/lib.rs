struct Transaction([u8]);

struct Event<H: Hash> {
    self_parent: H,
    other_parent: H,
    txs: Vec<Transaction>,
    round: usize,
    witness: bool,
}
