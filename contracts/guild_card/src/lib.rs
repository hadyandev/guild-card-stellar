#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Address, Map
};

// Guild Card Data Structure
#[contracttype]
#[derive(Clone)]
pub struct GuildCard {
    pub owner: Address,           // Wallet owner
    pub guild_name: String,       // Guild/organization name  
    pub member_name: String,      // Member display name
    pub exp: u64,                 // Experience points
    pub rank: String,             // Current rank (C, B, A, S)
    pub joined_at: u64,           // Join timestamp
}

// Storage keys
const GUILD_DATA: soroban_sdk::Symbol = soroban_sdk::symbol_short!("GUILD");

#[contract]
pub struct GuildContract;

#[contractimpl]
impl GuildContract {
    // Create new guild membership card (Soulbound - non-transferable)
    pub fn mint(env: Env, owner: Address, guild_name: String, member_name: String) -> String {
        // Verify authorization
        owner.require_auth();
        
        // Check if already has card
        let key = (GUILD_DATA, owner.clone());
        if env.storage().instance().has(&key) {
            return String::from_str(&env, "Already has guild card");
        }
        
        // Create new card
        let card = GuildCard {
            owner: owner.clone(),
            guild_name,
            member_name,
            exp: 0,
            rank: String::from_str(&env, "C"), // Start at C rank
            joined_at: env.ledger().timestamp(),
        };
        
        // Store card
        env.storage().instance().set(&key, &card);
        
        String::from_str(&env, "Guild card minted!")
    }
    
    // Get guild card info
    pub fn get(env: Env, owner: Address) -> GuildCard {
        let key = (GUILD_DATA, owner);
        env.storage().instance().get(&key).unwrap_or(GuildCard {
            owner: env.current_contract_address(),
            guild_name: String::from_str(&env, ""),
            member_name: String::from_str(&env, "Not found"),
            exp: 0,
            rank: String::from_str(&env, "-"),
            joined_at: 0,
        })
    }
    
    // Add experience points
    pub fn add_exp(env: Env, owner: Address, amount: u64) -> String {
        owner.require_auth();
        
        let key = (GUILD_DATA, owner.clone());
        if let Some(card) = env.storage().instance().get::<(soroban_sdk::Symbol, Address), GuildCard>(&key) {
            let new_exp = card.exp + amount;
            
            // Determine new rank based on EXP
            let new_rank = if new_exp >= 10000 {
                String::from_str(&env, "S")
            } else if new_exp >= 5000 {
                String::from_str(&env, "A")
            } else if new_exp >= 1000 {
                String::from_str(&env, "B")
            } else {
                String::from_str(&env, "C")
            };
            
            let updated = GuildCard {
                exp: new_exp,
                rank: new_rank,
                ..card
            };
            
            env.storage().instance().set(&key, &updated);
            return String::from_str(&env, "EXP added! Level up check with get_rank()");
        }
        
        String::from_str(&env, "No guild card found")
    }
    
    // Get current rank
    pub fn get_rank(env: Env, owner: Address) -> String {
        let card = Self::get(env, owner);
        card.rank
    }
    
    // Get total EXP
    pub fn get_exp(env: Env, owner: Address) -> u64 {
        let card = Self::get(env, owner);
        card.exp
    }
}

// Test module
#[cfg(test)]
mod test;
