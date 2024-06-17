use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item,Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address : Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Agreement {
    pub id : u64,
    pub owner: Addr, //
    pub price : i128, //6
    pub initial_amount: i128, //3
    pub final_amount : i128, //2
    pub start_time: u64, //7
    pub end_time: u64, //1
    pub withdrawn: bool, //10
    pub on_market : bool, //4
    pub timeleft : u64, //8
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const AGREEMENT: Map<u64,Agreement> = Map::new("agreement");

