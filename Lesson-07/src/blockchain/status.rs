#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum TransactionStatus {
    Pending,
    Completed,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum BlockStatus {
    InProcess,
    Mined,
}
