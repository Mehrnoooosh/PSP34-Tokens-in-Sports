#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buy_ticket_test() {
        let mut contract = MyPSP34::new(1000);

        let user1 = AccountId::from([1; 32]);
        let user2 = AccountId::from([2; 32]);

        // User 1 buys a ticket
        let result = contract.buy_ticket(0, 100, user1.clone());
        assert_eq!(result, Ok(()));
        assert_eq!(contract.tickets[0].owner, Some(user1.clone()));
        assert_eq!(contract.tickets[0].price, 100);

        // User 2 tries to buy the same ticket with insufficient funds
        let result = contract.buy_ticket(0, 200, user2.clone());
        assert_eq!(result, Err(Error::InsufficientFunds));

        // User 2 tops up their balance
        let result = contract.top_up_balance(200, user2.clone());
        assert_eq!(result, Ok(()));
        assert_eq!(contract.balances[&user2], 200);

        // User 2 buys the ticket successfully
        let result = contract.buy_ticket(0, 150, user2.clone());
        assert_eq!(result, Ok(()));
        assert_eq!(contract.tickets[0].owner, Some(user2.clone()));
        assert_eq!(contract.tickets[0].price, 150);
    }
}
