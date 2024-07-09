use std::str::FromStr;

use cosmwasm_std::{to_json_binary, Binary, Deps, Empty, Env, QueryRequest, StdResult, Uint256};

use crate::msgs::QueryMsg;
use crate::slinky_oracle::{
    CurrencyPair, GetAllCurrencyPairsRequest, GetPriceRequest, GetPricesRequest,
};
use crate::state::Contract;
use crate::timestamp::convert_iso_string_to_timestamp;
use protobuf::{Message, MessageField};
use slinky_wasm::oracle::{
    GetAllCurrencyPairsResponse, GetPriceResponse, GetPricesResponse, QuotePrice,
};

impl<'a> Contract {
    fn get_price(
        &self,
        deps: Deps,
        _env: Env,
        base: String,
        quote: String,
    ) -> StdResult<GetPriceResponse> {
        let request = GetPriceRequest {
            currency_pair: MessageField::some(CurrencyPair {
                Base: base,
                Quote: quote,
                special_fields: ::protobuf::SpecialFields::new(),
            }),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        let bytes = request.write_to_bytes().unwrap();

        let data = Binary::from(bytes);
        let request = QueryRequest::Stargate {
            path: "/slinky.oracle.v1.Query/GetPrice".to_string(),
            data,
        };
        let res: GetPriceResponseRaw = deps.querier.query(&request)?;
        Ok(convert_raw_price_response(&res))
    }

    fn get_prices(
        &self,
        deps: Deps,
        _env: Env,
        pair_ids: Vec<String>,
    ) -> StdResult<GetPricesResponse> {
        let request = GetPricesRequest {
            currency_pair_ids: pair_ids,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        let bytes = request.write_to_bytes().unwrap();

        let data = Binary::from(bytes);
        let request = QueryRequest::Stargate {
            path: "/slinky.oracle.v1.Query/GetPrices".to_string(),
            data,
        };
        let raw_res: GetPricesResponseRaw = deps.querier.query(&request)?;
        let res = GetPricesResponse {
            prices: raw_res
                .prices
                .into_iter()
                .map(|raw| convert_raw_price_response(&raw))
                .collect(),
        };
        Ok(res)
    }
    fn get_all_currency_pairs(
        &self,
        deps: Deps,
        _env: Env,
    ) -> StdResult<GetAllCurrencyPairsResponse> {
        let request = GetAllCurrencyPairsRequest {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        let bytes = request.write_to_bytes().unwrap();

        let data = Binary::from(bytes);
        let request = QueryRequest::<Empty>::Stargate {
            path: "/slinky.oracle.v1.Query/GetAllCurrencyPairs".to_string(),
            data,
        };
        let res: GetAllCurrencyPairsResponse = deps.querier.query(&request)?;
        Ok(res)
    }
}

impl<'a> Contract {
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetPrice { base, quote } => {
                to_json_binary(&self.get_price(deps, env, base, quote)?)
            }
            QueryMsg::GetPrices { pair_ids } => {
                to_json_binary(&self.get_prices(deps, env, pair_ids)?)
            }
            QueryMsg::GetAllCurrencyPairs {} => {
                to_json_binary(&self.get_all_currency_pairs(deps, env)?)
            }
        }
    }
}

fn convert_raw_price_response(raw_response: &GetPriceResponseRaw) -> GetPriceResponse {
    GetPriceResponse {
        price: QuotePrice {
            price: Uint256::from_str(&raw_response.price.price).unwrap(),
            block_timestamp: convert_iso_string_to_timestamp(&raw_response.price.block_timestamp),
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
    pub prices: Vec<GetPriceResponseRaw>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct QuotePriceRaw {
    pub price: String,
    pub block_timestamp: String,
    pub block_height: String,
}
