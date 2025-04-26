#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod luckydot {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use ink::env::hash::{Blake2x256, HashOutput};

    #[ink(storage)]
    pub struct LuckyDot {
        tickets: Mapping<AccountId, Vec<Ticket>>,
        prize_pool: Balance,
        ticket_price: Balance,
        admin: AccountId,
    }

    #[derive(scale::Encode, scale::Decode, Clone, PartialEq, Eq, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Ticket {
        id: u64,
        scratched: bool,
        symbols: [u8; 3], // 0:🍒 1:🍀 2:💎 3:🍋
        prize_multiplier: u8, // 0 = no win, 1 = small win, 2 = big win
    }

    impl LuckyDot {
        #[ink(constructor)]
        pub fn new(ticket_price: Balance) -> Self {
            let caller = Self::env().caller();
            Self {
                tickets: Mapping::default(),
                prize_pool: 0,
                ticket_price,
                admin: caller,
            }
        }

        #[ink(message, payable)]
        pub fn buy_ticket(&mut self) -> Result<(), String> {
            let caller = self.env().caller();
            let value = self.env().transferred_value();
            if value < self.ticket_price {
                return Err(String::from("Insufficient payment"));
            }

            let block_hash = self.env().block_hash(self.env().block_number());
            let mut rng = <Blake2x256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Blake2x256>(&block_hash, &mut rng);

            let symbols = [
                rng[0] % 4, // 4 different symbols
                rng[1] % 4,
                rng[2] % 4,
            ];

            let ticket_id = self.env().block_number() as u64 + value as u64;

            let ticket = Ticket {
                id: ticket_id,
                scratched: false,
                symbols,
                prize_multiplier: 0, // not scratched yet
            };

            let mut user_tickets = self.tickets.get(&caller).unwrap_or_default();
            user_tickets.push(ticket);
            self.tickets.insert(caller, &user_tickets);

            self.prize_pool += value;

            Ok(())
        }

        #[ink(message)]
        pub fn scratch_ticket(&mut self, ticket_id: u64) -> Result<[u8; 3], String> {
            let caller = self.env().caller();
            let mut user_tickets = self.tickets.get(&caller).ok_or("No tickets found")?;

            for ticket in user_tickets.iter_mut() {
                if ticket.id == ticket_id {
                    if ticket.scratched {
                        return Err(String::from("Ticket already scratched"));
                    }

                    let prize = Self::calculate_prize(ticket.symbols);
                    ticket.prize_multiplier = prize;
                    ticket.scratched = true;

                    self.tickets.insert(caller, &user_tickets);

                    return Ok(ticket.symbols);
                }
            }

            Err(String::from("Ticket not found"))
        }

        #[ink(message)]
        pub fn claim_prize(&mut self, ticket_id: u64) -> Result<(), String> {
            let caller = self.env().caller();
            let mut user_tickets = self.tickets.get(&caller).ok_or("No tickets found")?;

            for ticket in user_tickets.iter_mut() {
                if ticket.id == ticket_id {
                    if !ticket.scratched {
                        return Err(String::from("Ticket not scratched yet"));
                    }
                    if ticket.prize_multiplier == 0 {
                        return Err(String::from("No prize to claim"));
                    }

                    let prize_amount = self.ticket_price * ticket.prize_multiplier as u128;
                    ticket.prize_multiplier = 0; // reset prize to prevent double-claim
                    self.tickets.insert(caller, &user_tickets);

                    if self.env().transfer(caller, prize_amount).is_err() {
                        return Err(String::from("Prize transfer failed"));
                    }

                    return Ok(());
                }
            }

            Err(String::from("Ticket not found"))
        }

        #[ink(message)]
        pub fn get_tickets(&self, user: AccountId) -> Vec<Ticket> {
            self.tickets.get(&user).unwrap_or_default()
        }

        fn calculate_prize(symbols: [u8; 3]) -> u8 {
            if symbols[0] == symbols[1] && symbols[1] == symbols[2] {
                2 // Big prize
            } else if symbols[0] == symbols[1] || symbols[0] == symbols[2] || symbols[1] == symbols[2] {
                1 // Small prize
            } else {
                0 // No prize
            }
        }
    }
}