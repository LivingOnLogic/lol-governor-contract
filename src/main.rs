#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod lol_governor_contract {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct LolGovernorContract {
        votes: Mapping<AccountId, bool>,
        proposal: Hash,
        voting_open: bool,
    }

    impl LolGovernorContract {
        #[ink(constructor)]
        pub fn new(proposal: Hash) -> Self {
            Self {
                votes: Mapping::default(),
                proposal,
                voting_open: true,
            }
        }

        #[ink(message)]
        pub fn vote(&mut self, approve: bool) -> Result<(), &'static str> {
            let caller = self.env().caller();
            if !self.voting_open {
                return Err("Voting is closed");
            }
            if self.votes.contains(caller) {
                return Err("Already voted");
            }
            self.votes.insert(caller, &approve);
            Ok(())
        }

        #[ink(message)]
        pub fn close_voting(&mut self) {
            self.voting_open = false;
        }

        #[ink(message)]
        pub fn result(&self) -> (u32, u32) {
            let mut yes = 0;
            let mut no = 0;
            for (k, v) in self.votes.iter() {
                if v {
                    yes += 1;
                } else {
                    no += 1;
                }
            }
            (yes, no)
        }

        #[ink(message)]
        pub fn get_proposal(&self) -> Hash {
            self.proposal
        }
    }
}
