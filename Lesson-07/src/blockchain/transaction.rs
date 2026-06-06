

use crate::blockchain::status::{TransactionStatus};
#[derive(Debug)]
pub struct Transaction {
    pub transaction_id: u64,
    amount: u64,
    sender: String,
    receiver: String,
    timestamp: u64,
   pub  status: TransactionStatus,
}




impl Transaction {
    pub fn new(
        transaction_id: u64,
        amount: u64,
        sender: String,
        receiver: String,
        timestamp: u64,
        status: TransactionStatus,
    ) -> Transaction {
        Transaction {
            transaction_id,
            amount,
            sender,
            receiver,
            timestamp,
            status,
        }
    }

    pub fn print(&self) {
        println!("{:?} ", self);
    }

    pub fn transaction_status(&mut self, new_status: TransactionStatus) {
        self.status = new_status;
    }
}
