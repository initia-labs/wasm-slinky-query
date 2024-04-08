use std::str::FromStr;

use cosmwasm_std::{to_json_binary, Binary, Deps, Empty, Env, QueryRequest, StdResult, Uint256};

use crate::slinky_query_proto::{GetAllCurrencyPairsRequest, GetPricesRequest};
use crate::state::Contract;
use crate::msgs::QueryMsg;
use crate::slinky_query_proto::{GetPriceRequest, get_price_request::Currency_pair_selector, };
use protobuf::Message;

impl<'a> Contract {
    fn get_price(&self, deps: Deps, _env: Env, pair_id: String) -> StdResult<GetPriceResponse> {
        let request = GetPriceRequest { 
            currency_pair_selector: Some(Currency_pair_selector::CurrencyPairId(pair_id)),
            special_fields: ::protobuf::SpecialFields::new()
        };
        let bytes = request.write_to_bytes().unwrap();
        
        let data = Binary::from(bytes);
        let request = QueryRequest::Stargate{path: "/slinky.oracle.v1.Query/GetPrice".to_string(), data};
        let res: GetPriceResponseRaw = deps.querier.query(&request)?;
        Ok(convert_raw_price_response(&res))
    }
    fn get_prices(&self, deps: Deps, _env: Env, pair_ids: Vec<String>) -> StdResult<GetPricesResponse> {
        let request = GetPricesRequest { 
            currency_pair_ids: pair_ids,
            special_fields: ::protobuf::SpecialFields::new()
        };
        let bytes = request.write_to_bytes().unwrap();
        
        let data = Binary::from(bytes);
        let request = QueryRequest::Stargate{path: "/slinky.oracle.v1.Query/GetPrices".to_string(), data};
        let raw_res: GetPricesResponseRaw = deps.querier.query(&request)?;
        let res = GetPricesResponse {
            prices: raw_res.prices.into_iter().map(|raw| convert_raw_price_response(&raw)).collect()
        };
        Ok(res)
    }
    fn get_all_currency_pairs(&self, deps: Deps, _env: Env) -> StdResult<GetAllCurrencyPairsResponse> {
        let request = GetAllCurrencyPairsRequest { 
            special_fields: ::protobuf::SpecialFields::new()
        };
        let bytes = request.write_to_bytes().unwrap();

        let data = Binary::from(bytes);
        let request = QueryRequest::<Empty>::Stargate{path: "/slinky.oracle.v1.Query/GetAllCurrencyPairs".to_string(), data};
        let res: GetAllCurrencyPairsResponse = deps.querier.query(&request)?;
        Ok(res)
    }
}


impl<'a> Contract {
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetPrice { pair_id } => to_json_binary(&self.get_price(deps, env, pair_id)?),
            QueryMsg::GetPrices { pair_ids } => to_json_binary(&self.get_prices(deps, env, pair_ids)?),
            QueryMsg::GetAllCurrencyPairs {} => to_json_binary(&self.get_all_currency_pairs(deps, env)?),
        }
    }
}

fn convert_raw_price_response(raw_response: &GetPriceResponseRaw) -> GetPriceResponse {
    GetPriceResponse {
        price: QuotePrice {
            price: Uint256::from_str(&raw_response.price.price).unwrap(),
            block_timestamp: raw_response.price.block_timestamp.clone(),
            block_height: u64::from_str(&raw_response.price.block_height).unwrap(),
        },
        nonce: u64::from_str(&raw_response.nonce).unwrap(),
        decimals: u64::from_str(&raw_response.decimals).unwrap(),
        id: u64::from_str(&raw_response.id).unwrap(),
    }
}

// stargate query responses

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetPriceResponseRaw {
    pub price: QuotePriceRaw,
    pub nonce: String,
    pub decimals: String,
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetPricesResponseRaw {
    pub prices: Vec<GetPriceResponseRaw>
}


#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct QuotePriceRaw {
    pub price: String,
    pub block_timestamp: String,
    pub block_height: String,
}

// query response

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetPriceResponse {
    pub price: QuotePrice,
    pub nonce: u64,
    pub decimals: u64,
    pub id: u64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetPricesResponse {
    pub prices: Vec<GetPriceResponse>
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct QuotePrice {
    pub price: Uint256,
    pub block_timestamp: String, // TODO: change this comsos_std::Timestamp
    pub block_height: u64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetAllCurrencyPairsResponse {
    pub currency_pairs: Vec<CurrencyPair>,
}
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[allow(non_snake_case)]
pub struct CurrencyPair {
    pub Base: String,
    pub Quote: String,
}