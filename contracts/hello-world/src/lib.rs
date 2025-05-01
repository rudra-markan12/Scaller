#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, String, symbol_short, log};

// Struct for Research Submission
#[contracttype]
#[derive(Clone)]
pub struct Research {
    pub id: u64,
    pub title: String,
    pub summary: String,
    pub researcher: String,
    pub accepted: bool,
}

// Constant symbol for ID tracking
const RESEARCH_COUNT: Symbol = symbol_short!("RSCH_CNT");

// Enum for key-value mapping
#[contracttype]
pub enum ResearchBook {
    Research(u64),
}

#[contract]
pub struct ResearchMarketplace;

#[contractimpl]
impl ResearchMarketplace {
    // Submit a new research proposal
    pub fn submit_research(env: Env, title: String, summary: String, researcher: String) -> u64 {
        let mut count = env.storage().instance().get(&RESEARCH_COUNT).unwrap_or(0);
        count += 1;

        let research = Research {
            id: count,
            title,
            summary,
            researcher,
            accepted: false,
        };

        env.storage().instance().set(&ResearchBook::Research(count), &research);
        env.storage().instance().set(&RESEARCH_COUNT, &count);

        log!(&env, "New research submitted: ID {}", count);
        count
    }

    // Accept a research proposal (by admin/mod)
    pub fn accept_research(env: Env, id: u64) {
        let mut r = Self::view_research(env.clone(), id);

        if r.accepted {
            panic!("Already accepted!");
        }

        r.accepted = true;
        env.storage().instance().set(&ResearchBook::Research(id), &r);

        log!(&env, "Research ID {} accepted.", id);
    }

    // View research by ID
    pub fn view_research(env: Env, id: u64) -> Research {
        env.storage().instance().get(&ResearchBook::Research(id)).unwrap_or(Research {
            id: 0,
            title: String::from_str(&env, "Not Found"),
            summary: String::from_str(&env, "Not Found"),
            researcher: String::from_str(&env, "Unknown"),
            accepted: false,
        })
    }
}
