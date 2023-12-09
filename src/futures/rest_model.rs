use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::rest_model::{string_or_bool, as_decimal, as_decimal_opt, string_or_u64};
pub use crate::rest_model::{
    Asks, Bids, BookTickers, KlineSummaries, KlineSummary, OrderSide, OrderStatus, RateLimit, ServerTime, SymbolPrice,
    SymbolStatus, Tickers, TimeInForce,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub futures_type: String,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<Filters>,
    pub assets: Vec<AssetDetail>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetail {
    pub asset: String,
    pub margin_available: bool,
    #[serde(with = "as_decimal")]
    pub auto_asset_exchange: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub pair: String,
    pub contract_type: ContractType,
    pub delivery_date: u64,
    pub onboard_date: u64,
    pub status: SymbolStatus,
    #[serde(with = "as_decimal")]
    pub maint_margin_percent: Decimal,
    #[serde(with = "as_decimal")]
    pub required_margin_percent: Decimal,
    pub base_asset: String,
    pub quote_asset: String,
    pub margin_asset: String,
    pub price_precision: u16,
    pub quantity_precision: u16,
    pub base_asset_precision: u64,
    pub quote_precision: u64,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
    pub settle_plan: u64,
    #[serde(with = "as_decimal")]
    pub trigger_protect: Decimal,
    pub filters: Vec<Filters>,
    pub order_types: Vec<OrderType>,
    pub time_in_force: Vec<TimeInForce>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

/// By default, use market orders
impl Default for OrderType {
    fn default() -> Self {
        Self::Market
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MarginType {
    Isolated,
    Cross,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum Filters {
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        #[serde(with = "as_decimal")]
        min_price: Decimal,
        #[serde(with = "as_decimal")]
        max_price: Decimal,
        #[serde(with = "as_decimal")]
        tick_size: Decimal,
    },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize {
        #[serde(with = "as_decimal")]
        min_qty: Decimal,
        #[serde(with = "as_decimal")]
        max_qty: Decimal,
        #[serde(with = "as_decimal")]
        step_size: Decimal,
    },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { limit: u16 },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { limit: u16 },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional {
        #[serde(with = "as_decimal")]
        notional: Decimal,
    },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        #[serde(with = "as_decimal")]
        multiplier_up: Decimal,
        #[serde(with = "as_decimal")]
        multiplier_down: Decimal,
        #[serde(with = "as_decimal")]
        multiplier_decimal: Decimal,
    },
    #[serde(other)]
    Others,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    // Undocumented
    #[serde(rename = "E")]
    pub event_time: u64,
    // Undocumented
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    #[serde(with = "as_decimal")]
    pub last_price: Decimal,
    #[serde(with = "as_decimal")]
    pub open_price: Decimal,
    #[serde(with = "as_decimal")]
    pub high_price: Decimal,
    #[serde(with = "as_decimal")]
    pub low_price: Decimal,
    #[serde(with = "as_decimal")]
    pub volume: Decimal,
    #[serde(with = "as_decimal")]
    pub quote_volume: Decimal,
    #[serde(with = "as_decimal")]
    pub last_qty: Decimal,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Trades {
    AllTrades(Vec<Trade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub is_buyer_maker: bool,
    #[serde(with = "as_decimal")]
    pub price: Decimal,
    #[serde(with = "as_decimal")]
    pub qty: Decimal,
    #[serde(with = "as_decimal")]
    pub quote_qty: Decimal,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AggTrades {
    AllAggTrades(Vec<AggTrade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggTrade {
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "a")]
    pub agg_id: u64,
    #[serde(rename = "f")]
    pub first_id: u64,
    #[serde(rename = "l")]
    pub last_id: u64,
    #[serde(rename = "m")]
    pub maker: bool,
    #[serde(rename = "p", with = "as_decimal")]
    pub price: Decimal,
    #[serde(rename = "q", with = "as_decimal")]
    pub qty: Decimal,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(untagged)]
// pub enum MarkPrices {
//     AllMarkPrices(Vec<MarkPrice>),
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub symbol: String,
    #[serde(with = "as_decimal")]
    pub mark_price: Decimal,
    #[serde(with = "as_decimal")]
    pub index_price: Decimal,
    #[serde(with = "as_decimal")]
    pub estimated_settle_price: Decimal,
    #[serde(with = "as_decimal")]
    pub last_funding_rate: Decimal,
    pub next_funding_time: u64,
    #[serde(with = "as_decimal")]
    pub interest_rate: Decimal,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LiquidationOrders {
    AllLiquidationOrders(Vec<LiquidationOrder>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationOrder {
    #[serde(with = "as_decimal")]
    pub average_price: Decimal,
    #[serde(with = "as_decimal")]
    pub executed_qty: Decimal,
    #[serde(with = "as_decimal")]
    pub orig_qty: Decimal,
    #[serde(with = "as_decimal")]
    pub price: Decimal,
    pub side: String,
    pub status: String,
    pub symbol: String,
    pub time: u64,
    pub time_in_force: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    #[serde(with = "as_decimal")]
    pub open_interest: Decimal,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub client_order_id: String,
    #[serde(with = "as_decimal")]
    pub cum_quote: Decimal,
    #[serde(with = "as_decimal")]
    pub executed_qty: Decimal,
    pub order_id: u64,
    #[serde(with = "as_decimal")]
    pub avg_price: Decimal,
    #[serde(with = "as_decimal")]
    pub orig_qty: Decimal,
    #[serde(with = "as_decimal")]
    pub price: Decimal,
    pub side: OrderSide,
    pub reduce_only: bool,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    #[serde(with = "as_decimal", default = "default_stop_price")]
    pub stop_price: Decimal,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub orig_type: OrderType,
    #[serde(with = "as_decimal", default = "default_activation_price")]
    pub activate_price: Decimal,
    #[serde(with = "as_decimal", default = "default_price_rate")]
    pub price_rate: Decimal,
    pub update_time: u64,
    pub working_type: WorkingType,
    pub price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub client_order_id: String,
    #[serde(with = "as_decimal")]
    pub cum_qty: Decimal,
    #[serde(with = "as_decimal")]
    pub cum_quote: Decimal,
    #[serde(with = "as_decimal")]
    pub executed_qty: Decimal,
    pub order_id: u64,
    #[serde(with = "as_decimal")]
    pub avg_price: Decimal,
    #[serde(with = "as_decimal")]
    pub orig_qty: Decimal,
    pub reduce_only: bool,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    #[serde(with = "as_decimal")]
    pub stop_price: Decimal,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub type_name: OrderType,
    pub orig_type: OrderType,
    #[serde(default)]
    #[serde(with = "as_decimal_opt")]
    pub activate_price: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "as_decimal_opt")]
    pub price_rate: Option<Decimal>,
    pub update_time: u64,
    pub working_type: WorkingType,
    price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrder {
    pub client_order_id: String,
    #[serde(with = "as_decimal")]
    pub cum_qty: Decimal,
    #[serde(with = "as_decimal")]
    pub cum_quote: Decimal,
    #[serde(with = "as_decimal")]
    pub executed_qty: Decimal,
    pub order_id: u64,
    #[serde(with = "as_decimal")]
    pub orig_qty: Decimal,
    pub orig_type: String,
    #[serde(with = "as_decimal")]
    pub price: Decimal,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "as_decimal")]
    pub stop_price: Decimal,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    #[serde(with = "as_decimal_opt")]
    pub activate_price: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "as_decimal_opt")]
    pub price_rate: Option<Decimal>,
    pub update_time: u64,
    pub working_type: String,
    price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(with = "as_decimal")]
    pub entry_price: Decimal,
    pub margin_type: MarginType,
    #[serde(with = "string_or_bool")]
    pub is_auto_add_margin: bool,
    #[serde(with = "as_decimal")]
    pub isolated_margin: Decimal,
    pub leverage: u64,
    #[serde(with = "as_decimal")]
    pub liquidation_price: Decimal,
    #[serde(with = "as_decimal")]
    pub mark_price: Decimal,
    #[serde(with = "as_decimal")]
    pub max_notional_value: Decimal,
    #[serde(with = "as_decimal", rename = "positionAmt")]
    pub position_amount: Decimal,
    pub symbol: String,
    #[serde(with = "as_decimal", rename = "unRealizedProfit")]
    pub unrealized_profit: Decimal,
    pub position_side: PositionSide,
    pub update_time: u64,
    #[serde(with = "as_decimal")]
    pub notional: Decimal,
    #[serde(with = "as_decimal")]
    pub isolated_wallet: Decimal,
}

// https://binance-docs.github.io/apidocs/futures/en/#account-information-v2-user_data
// it has differences from Position returned by positionRisk endpoint
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountPosition {
    pub symbol: String,
    #[serde(with = "as_decimal")]
    pub initial_margin: Decimal,
    #[serde(with = "as_decimal", rename = "maintMargin")]
    pub maintenance_margin: Decimal,
    #[serde(with = "as_decimal")]
    pub unrealized_profit: Decimal,
    #[serde(with = "as_decimal")]
    pub position_initial_margin: Decimal,
    #[serde(with = "as_decimal")]
    pub open_order_initial_margin: Decimal,
    #[serde(with = "string_or_u64")]
    pub leverage: u64,
    pub isolated: bool,
    #[serde(with = "as_decimal")]
    pub entry_price: Decimal,
    #[serde(with = "as_decimal")]
    pub max_notional: Decimal,
    #[serde(with = "as_decimal")]
    pub bid_notional: Decimal,
    #[serde(with = "as_decimal")]
    pub ask_notional: Decimal,
    pub position_side: PositionSide,
    #[serde(with = "as_decimal", rename = "positionAmt")]
    pub position_amount: Decimal,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountAsset {
    pub asset: String,
    #[serde(with = "as_decimal")]
    pub wallet_balance: Decimal,
    #[serde(with = "as_decimal")]
    pub unrealized_profit: Decimal,
    #[serde(with = "as_decimal")]
    pub margin_balance: Decimal,
    #[serde(with = "as_decimal")]
    pub maint_margin: Decimal,
    #[serde(with = "as_decimal")]
    pub initial_margin: Decimal,
    #[serde(with = "as_decimal")]
    pub position_initial_margin: Decimal,
    #[serde(with = "as_decimal")]
    pub open_order_initial_margin: Decimal,
    #[serde(with = "as_decimal")]
    pub cross_wallet_balance: Decimal,
    #[serde(with = "as_decimal", rename = "crossUnPnl")]
    pub cross_unrealized_pnl: Decimal,
    #[serde(with = "as_decimal")]
    pub available_balance: Decimal,
    #[serde(with = "as_decimal")]
    pub max_withdraw_amount: Decimal,
    pub margin_available: bool,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub fee_tier: u64,
    pub can_trade: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub update_time: u64,
    pub multi_assets_margin: bool,
    #[serde(with = "as_decimal")]
    pub total_initial_margin: Decimal,
    #[serde(with = "as_decimal", rename = "totalMaintMargin")]
    pub total_maintenance_margin: Decimal,
    #[serde(with = "as_decimal")]
    pub total_wallet_balance: Decimal,
    #[serde(with = "as_decimal")]
    pub total_unrealized_profit: Decimal,
    #[serde(with = "as_decimal")]
    pub total_margin_balance: Decimal,
    #[serde(with = "as_decimal")]
    pub total_position_initial_margin: Decimal,
    #[serde(with = "as_decimal")]
    pub total_open_order_initial_margin: Decimal,
    #[serde(with = "as_decimal")]
    pub total_cross_wallet_balance: Decimal,
    #[serde(with = "as_decimal", rename = "totalCrossUnPnl")]
    pub total_cross_unrealized_pnl: Decimal,
    #[serde(with = "as_decimal")]
    pub available_balance: Decimal,
    #[serde(with = "as_decimal")]
    pub max_withdraw_amount: Decimal,
    pub assets: Vec<AccountAsset>,
    pub positions: Vec<AccountPosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_alias: String,
    pub asset: String,
    #[serde(with = "as_decimal")]
    pub balance: Decimal,
    #[serde(with = "as_decimal")]
    pub cross_wallet_balance: Decimal,
    #[serde(with = "as_decimal", rename = "crossUnPnl")]
    pub cross_unrealized_pnl: Decimal,
    #[serde(with = "as_decimal")]
    pub available_balance: Decimal,
    #[serde(with = "as_decimal")]
    pub max_withdraw_amount: Decimal,
    pub margin_available: bool,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageResponse {
    pub leverage: u8,
    #[serde(with = "as_decimal")]
    pub max_notional_value: Decimal,
    pub symbol: String,
}

fn default_stop_price() -> Decimal {
    Decimal::default()
}
fn default_activation_price() -> Decimal {
    Decimal::default()
}
fn default_price_rate() -> Decimal {
    Decimal::default()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HistoryQuery {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_id: Option<u64>,
    pub limit: u16,
    pub symbol: String,
    pub interval: Option<String>,
    pub period: Option<String>,
}

impl HistoryQuery {
    pub fn validate(&self) -> crate::errors::Result<()> {
        if let Some(period) = &self.period {
            if !PERIODS.contains(&period.as_str()) {
                return Err(crate::errors::Error::InvalidPeriod(period.clone()));
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    pub funding_time: u64,
    #[serde(with = "as_decimal")]
    pub funding_rate: Decimal,
}

pub static PERIODS: &[&str] = &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"];

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHistory {
    pub symbol: String,
    #[serde(with = "as_decimal")]
    pub sum_open_interest: Decimal,
    #[serde(with = "as_decimal")]
    pub sum_open_interest_value: Decimal,
    pub timestamp: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    pub symbol: String,
    #[serde(with = "as_decimal")]
    pub long_account: Decimal,
    #[serde(with = "as_decimal")]
    pub long_short_ratio: Decimal,
    #[serde(with = "as_decimal")]
    pub short_account: Decimal,
    pub timestamp: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
    pub bracket: u8,
    pub initial_leverage: u8,
    pub notional_cap: u64,
    pub notional_floor: u64,
    pub maint_margin_ratio: Decimal,
    pub cum: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolBrackets {
    pub symbol: String,
    pub brackets: Vec<LeverageBracket>,
}
