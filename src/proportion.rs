/// Specifies an input proportion,
pub enum Proportion<N, P> {
    /// As number of successes.
    NSuccesses(N),

    /// As sample proportion.
    PHat(P),
}
