use ink_lang::contract;

#[contract]
mod sports {
    #[ink(storage)]
    pub struct Sports {
        owner: AccountId,
        ticket_prices: Vec<u64>,
        tickets_available: Vec<u64>,
        tickets_sold: Vec<u64>,
    }

    impl Sports {
        #[ink(constructor)]
        pub fn new(owner: AccountId, ticket_prices: Vec<u64>, tickets_available: Vec<u64>) -> Self {
            Self {
                owner,
                ticket_prices,
                tickets_available,
                tickets_sold: vec![0; ticket_prices.len()],
            }
        }

        #[ink(message)]
        pub fn buy_ticket(&mut self, index: u32) -> bool {
            let index = index as usize;
            if self.tickets_available[index] > self.tickets_sold[index] {
                self.tickets_sold[index] += 1;
                return true;
            }
            false
        }

        #[ink(message)]
        pub fn get_ticket_price(&self, index: u32) -> u64 {
            let index = index as usize;
            self.ticket_prices[index]
        }

        #[ink(message)]
        pub fn get_tickets_available(&self, index: u32) -> u64 {
            let index = index as usize;
            self.tickets_available[index] - self.tickets_sold[index]
        }

        #[ink(message)]
        pub fn get_tickets_sold(&self, index: u32) -> u64 {
            let index = index as usize;
            self.tickets_sold[index]
        }
    }
}