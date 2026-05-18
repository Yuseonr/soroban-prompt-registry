#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct PromptRecord {
    pub id: u64,
    pub category: String,
    pub content: String,
    pub owner: Address,
    pub timestamp: u64,
}

const PROMPT_DATA: Symbol = symbol_short!("PROMPT");

#[contract]
pub struct PromptRegistryContract;

#[contractimpl]
impl PromptRegistryContract {
    // READ - ambil semua prompt (publik)
    pub fn get_prompts(env: Env) -> Vec<PromptRecord> {
        env.storage().instance().get(&PROMPT_DATA).unwrap_or(Vec::new(&env))
    }

    // CREATE - register prompt baru
    pub fn register_prompt(env: Env, owner: Address, category: String, content: String) -> String {
        owner.require_auth();

        let mut prompts: Vec<PromptRecord> = env
            .storage()
            .instance()
            .get(&PROMPT_DATA)
            .unwrap_or(Vec::new(&env));

        let record = PromptRecord {
            id: env.prng().gen::<u64>(),
            category,
            content,
            owner,
            timestamp: env.ledger().timestamp(),
        };

        prompts.push_back(record);
        env.storage().instance().set(&PROMPT_DATA, &prompts);

        String::from_str(&env, "Prompt berhasil diregister")
    }

    // UPDATE - edit kategori dan/atau konten, hanya owner
    pub fn update_prompt(
        env: Env,
        owner: Address,
        id: u64,
        new_category: String,
        new_content: String,
    ) -> String {
        owner.require_auth();

        let mut prompts: Vec<PromptRecord> = env
            .storage()
            .instance()
            .get(&PROMPT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..prompts.len() {
            let record = prompts.get(i).unwrap();
            if record.id == id {
                if record.owner != owner {
                    return String::from_str(&env, "Tidak diizinkan: bukan pemilik prompt");
                }
                let updated = PromptRecord {
                    id: record.id,
                    category: new_category,
                    content: new_content,
                    owner: record.owner,
                    timestamp: record.timestamp, // timestamp tetap dari awal register
                };
                prompts.set(i, updated);
                env.storage().instance().set(&PROMPT_DATA, &prompts);
                return String::from_str(&env, "Prompt berhasil diupdate");
            }
        }

        String::from_str(&env, "Prompt tidak ditemukan")
    }

    // DELETE - hapus, hanya owner
    pub fn delete_prompt(env: Env, owner: Address, id: u64) -> String {
        owner.require_auth();

        let mut prompts: Vec<PromptRecord> = env
            .storage()
            .instance()
            .get(&PROMPT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..prompts.len() {
            let record = prompts.get(i).unwrap();
            if record.id == id {
                if record.owner != owner {
                    return String::from_str(&env, "Tidak diizinkan: bukan pemilik prompt");
                }
                prompts.remove(i);
                env.storage().instance().set(&PROMPT_DATA, &prompts);
                return String::from_str(&env, "Prompt berhasil dihapus");
            }
        }

        String::from_str(&env, "Prompt tidak ditemukan")
    }
}

mod test;