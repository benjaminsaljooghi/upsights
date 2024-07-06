use up_api::v1::accounts::ListAccountsOptions;
use up_api::v1::transactions::{ListTransactionsOptions, ListTransactionsResponse, TransactionResource};

pub struct Up {
    pub up_client: up_api::v1::Client,
}

impl Up {
    pub fn new(token: String) -> Self {
        let up_client = up_api::v1::Client::new(token);
        return Self {
            up_client
        };
    }

    pub async fn get_account_id(&self) -> String {
        let mut options = ListAccountsOptions::default();
        options.filter_account_type("TRANSACTIONAL".into());
        options.filter_ownership_type("JOINT".into());
        let accounts = &self.up_client.list_accounts(
            &options
        ).await.unwrap();
        let account_id = &accounts.data[0].id;
        return account_id.clone();
    }

    pub async fn get_transactions(&self, account_id: &String) -> ListTransactionsResponse {
        let options = ListTransactionsOptions::default();
        let mut transactions = self.up_client.list_transactions_by_account(
            account_id,
            &options
        ).await.unwrap();
        return transactions;
    }

    pub async fn get_spending(&self, account_id: &String) -> Vec<TransactionResource> {
        let mut transactions = self.get_transactions(account_id).await;
        let filtered: Vec<TransactionResource> = transactions
            .data
            .into_iter()
            .filter(|t| t.attributes.is_categorizable && t.attributes.amount.value_in_base_units < 0)
            .collect();
        return filtered;
    }
}