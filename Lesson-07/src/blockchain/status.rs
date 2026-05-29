#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum BlockStatus {
    InProcess,
    InValid,
    Mined,
    Orphaned,
}
