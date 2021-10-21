use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, AccountId, Balance, Timestamp, Promise, 
    json_types::{ U128, Base58PublicKey },};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PrizeContract {
    // SETUP CONTRACT STATE
    pub entries: Vec<AccountId>,
    pub entry_fee: Balance,
    pub close_date: Timestamp,
    pub prize_pool: Balance,
    pub open: bool,
    pub winner: String//AccountId
}

impl Default for PrizeContract {
    fn default() -> Self {
        panic!("Should be initialised before usage")
    }
}

#[near_bindgen]
impl PrizeContract {
    // ADD CONTRACT METHODS HERE
    #[init]
    pub fn new (){
        
    }


    #[payable]
    pub fn enter_draw(&mut self){
        if near_sdk::env::attached_deposit() == self.entry_fee{
            env::log_str("money mtaches add entry");
            self.prize_pool = self.prize_pool + (env::attached_deposit()/4)*3;
            self.entries.push(near_sdk::env::signer_account_id());
        }else{
            near_sdk::env::panic_str("Entry Fee not enough!!");
        }
        env::log_str("Entering the lottery");
    }
    pub fn get_prize_pool( self) -> u128{
        return self.prize_pool;
    }

    pub fn make_the_draw(&mut self)-> Promise {
        let rand_array = [*env::random_seed().get(0).unwrap(),*env::random_seed().get(2).unwrap(),*env::random_seed().get(3).unwrap(), *env::random_seed().get(4).unwrap(),*env::random_seed().get(5).unwrap()];
        let len:u128 = self.entries.len() as u128;
        let rand = (rand_array[0] + rand_array[1] + rand_array[2] + rand_array[3]+ rand_array[4]) as u128;
        let winner_ = &self.entries[(rand%len) as usize];
        self.winner = winner_.to_string();
        env::log_str("We have a winner: ");
        env::log_str(&winner_.to_string());
        Promise::new(self.winner.parse().unwrap()).transfer(self.prize_pool)
    }

    pub fn close_contract(self)->Promise{
        Promise::new("sockdrawer.testnet".parse().unwrap()).transfer(env::account_balance())
    }

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */


