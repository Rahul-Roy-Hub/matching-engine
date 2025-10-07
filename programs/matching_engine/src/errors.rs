use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("The computation was aborted")]
    AbortedComputation,
    #[msg("Cluster not set")]
    ClusterNotSet,
    #[msg("Order ID overflow")]
    OrderIdOverflow,
    #[msg("Matching can only occur every 15 seconds")]
    MatchingTooFrequent,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Overflow")]
    Overflow,
}
