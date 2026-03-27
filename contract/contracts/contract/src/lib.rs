#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
pub struct Domain {
    pub owner: Address,
    pub price: i128,
    pub is_for_sale: bool,
}

#[contract]
pub struct DomainMarketplace;

#[contractimpl]
impl DomainMarketplace {

    // Register a domain
    pub fn register(env: Env, name: String, owner: Address) {
        owner.require_auth();

        if env.storage().instance().has(&name) {
            panic!("Domain already exists");
        }

        let domain = Domain {
            owner: owner.clone(),
            price: 0,
            is_for_sale: false,
        };

        env.storage().instance().set(&name, &domain);
    }

    // List domain for sale
    pub fn list_for_sale(env: Env, name: String, owner: Address, price: i128) {
        owner.require_auth();

        let mut domain: Domain = match env.storage().instance().get(&name) {
            Some(d) => d,
            None => panic!("Domain not found"),
        };

        if domain.owner != owner {
            panic!("Not owner");
        }

        domain.price = price;
        domain.is_for_sale = true;

        env.storage().instance().set(&name, &domain);
    }

    // Buy domain
    pub fn buy(env: Env, name: String, buyer: Address) {
        buyer.require_auth();

        let mut domain: Domain = match env.storage().instance().get(&name) {
            Some(d) => d,
            None => panic!("Domain not found"),
        };

        if !domain.is_for_sale {
            panic!("Domain not for sale");
        }

        // Payment logic not implemented
        domain.owner = buyer;
        domain.is_for_sale = false;

        env.storage().instance().set(&name, &domain);
    }

    // Get domain details
    pub fn get_domain(env: Env, name: String) -> Domain {
        match env.storage().instance().get(&name) {
            Some(d) => d,
            None => panic!("Domain not found"),
        }
    }
}