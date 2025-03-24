use crate::futures::rest_model::{MarginType, OrderType, PositionSide, WorkingType};
use crate::rest_model::string_or_u64;
use crate::rest_model::{string_or_float, string_or_float_opt, ExecutionType, OrderSide, OrderStatus, TimeInForce};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "e")]
pub enum WebsocketEvent {
    #[serde(rename = "ACCOUNT_UPDATE")]
    AccountUpdate(Box<AccountUpdate>),
    OrderTradeUpdate(Box<OrderTradeUpdate>),
    #[serde(rename = "TRADE_LITE")]
    TradeLite(Box<TradeLite>),

    #[serde(rename = "bookTicker")]
    BookTicker(Box<BookTicker>),
    #[serde(rename = "trade")]
    Trade(Box<Trade>),
}

/// Book ticker event [Reference](https://developers.binance.com/docs/derivatives/usds-margined-futures/websocket-market-streams/Individual-Symbol-Book-Ticker-Streams)
///
/// example:
/// ```json
/// {
///   "e":"bookTicker",			// event type
///   "u":400900217,     		// order book updateId
///   "E": 1568014460893,  		// event time
///   "T": 1568014460891,  		// transaction time
///   "s":"BNBUSDT",     		// symbol
///   "b":"25.35190000", 		// best bid price
///   "B":"31.21000000", 		// best bid qty
///   "a":"25.36520000", 		// best ask price
///   "A":"40.66000000"  		// best ask qty
/// }
/// ```
///
///
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    #[serde(rename = "u", with = "string_or_u64")]
    pub update_id: u64,
    #[serde(rename = "E", with = "string_or_u64")]
    pub event_time: u64,
    #[serde(rename = "T", with = "string_or_u64")]
    pub transaction_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b", with = "string_or_float")]
    pub best_bid_price: f64,
    #[serde(rename = "B", with = "string_or_float")]
    pub best_bid_qty: f64,
    #[serde(rename = "a", with = "string_or_float")]
    pub best_ask_price: f64,
    #[serde(rename = "A", with = "string_or_float")]
    pub best_ask_qty: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    #[serde(rename = "E", with = "string_or_u64")]
    pub event_time: u64,
    #[serde(rename = "T", with = "string_or_u64")]
    pub transaction_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t", with = "string_or_u64")]
    pub trade_id: u64,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub quantity: f64,
    #[serde(rename = "X")]
    pub order_type: String, // Renamed from execution_type to be more descriptive
    #[serde(rename = "m")]
    pub is_maker_side: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeLite {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T", with = "string_or_u64")]
    pub transaction_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "q", with = "string_or_float")]
    pub original_quantity: f64,
    #[serde(rename = "p", with = "string_or_float")]
    pub original_price: f64,
    #[serde(rename = "m")]
    pub is_maker_side: bool,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: OrderSide,
    #[serde(rename = "L", with = "string_or_float")]
    pub last_filled_price: f64,
    #[serde(rename = "l", with = "string_or_float")]
    pub order_last_filled_quantity: f64,
    #[serde(rename = "t", with = "string_or_u64")]
    pub trade_id: u64,
    #[serde(rename = "i", with = "string_or_u64")]
    pub order_id: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "a")]
    pub account: Account,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    #[serde(rename = "m")]
    pub reason_type: ReasonType,
    #[serde(rename = "B")]
    pub balances: Vec<Balance>,
    #[serde(rename = "P")]
    pub positions: Vec<Position>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReasonType {
    Deposit,
    Withdraw,
    Order,
    FundingFee,
    WithdrawReject,
    Adjustment,
    InsuranceClear,
    AdminDeposit,
    AdminWithdraw,
    MarginTransfer,
    MarginTypeChange,
    AssetTransfer,
    OptionsPremiumFee,
    OptionsSettleProfit,
    AutoExchange,
    CoinSwapDeposit,
    CoinSwapWithdraw,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Balance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb", with = "string_or_float")]
    pub wallet_balance: f64,
    #[serde(rename = "cw", with = "string_or_float")]
    pub cross_wallet_balance: f64,
    #[serde(rename = "bc", with = "string_or_float")]
    pub balance_change: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa", with = "string_or_float")]
    pub position_amount: f64,
    #[serde(rename = "ep", with = "string_or_float")]
    pub entry_price: f64,
    #[serde(rename = "bep", with = "string_or_float")]
    pub breakeven_price: f64,
    #[serde(rename = "cr", with = "string_or_float")]
    pub accumulated_realized: f64,
    #[serde(rename = "up", with = "string_or_float")]
    pub unrealized_profit: f64,
    #[serde(rename = "mt")]
    pub margin_type: MarginType,
    #[serde(rename = "iw", with = "string_or_float")]
    pub isolated_wallet: f64,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrderTradeUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "o")]
    pub order: Order,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: OrderSide,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q", with = "string_or_float")]
    pub quantity: f64,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "ap", with = "string_or_float")]
    pub average_price: f64,
    #[serde(rename = "sp", with = "string_or_float")]
    pub stop_price: f64,
    #[serde(rename = "x")]
    pub execution_type: ExecutionType,
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l", with = "string_or_float")]
    pub order_last_filled_quantity: f64,
    #[serde(rename = "z", with = "string_or_float")]
    pub order_filled_accumulated_quantity: f64,
    #[serde(rename = "L", with = "string_or_float")]
    pub last_filled_price: f64,
    #[serde(default, rename = "n", with = "string_or_float_opt")]
    pub commission: Option<f64>,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "T")]
    pub order_trade_time: u64,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "b", with = "string_or_float")]
    pub bid_notional: f64,
    #[serde(rename = "a", with = "string_or_float")]
    pub ask_notional: f64,
    #[serde(rename = "m")]
    pub is_maker: bool,
    #[serde(rename = "R")]
    pub is_reduce: bool,
    #[serde(rename = "wt")]
    pub working_type: WorkingType,
    #[serde(rename = "ot")]
    pub original_order_type: OrderType,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
    #[serde(rename = "cp")]
    pub close_position: bool,
    #[serde(default, rename = "AP", with = "string_or_float_opt")]
    pub activation_price: Option<f64>,
    #[serde(default, rename = "cr", with = "string_or_float_opt")]
    pub callback_rate: Option<f64>,
    #[serde(rename = "pP")]
    pub price_protect: bool,
    #[serde(rename = "rp", with = "string_or_float")]
    pub realized_profit: f64,
    #[serde(rename = "V")]
    pub stp_mode: SelfTradePreventionMode,
    #[serde(rename = "pm")]
    pub price_match: PriceMatch,
    #[serde(rename = "gtd")]
    pub good_till_date: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceMatch {
    /// No price match
    None,
    /// Counterparty best price
    Opponent,
    /// The 5th best price from the counterparty
    Opponent5,
    /// The 10th best price from the counterparty
    Opponent10,
    /// The 20th best price from the counterparty
    Opponent20,
    /// The best price on the same side of the order book
    Queue,
    /// The 5th best price on the same side of the order book
    Queue5,
    /// The 10th best price on the same side of the order book
    Queue10,
    /// The 20th best price on the same side of the order book
    Queue20,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    /// No Self-Trade Prevention
    None,
    /// Expire taker order when STP trigger
    ExpireTaker,
    /// Expire taker and maker order when STP trigger
    ExpireBoth,
    /// Expire maker order when STP trigger
    ExpireMaker,
}

#[derive(thiserror::Error, Debug)]
enum ParseError {
    #[error("JSON parsing error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Unknown event type: {0}")]
    UnknownEventType(String),
    // Add other potential error types here
}

fn parse_event(event_str: &str) -> Result<WebsocketEvent, ParseError> {
    // First, try to deserialize into the generic BinanceEvent enum.
    let event: WebsocketEvent = serde_json::from_str(event_str)?;
    Ok(event)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_event() {
        let book_ticker_str = r#"{"e":"bookTicker","u":7081766043288,"s":"BTCUSDT","b":"84158.30","B":"6.454","a":"84158.40","A":"9.533","T":1742582931601,"E":1742582931601}"#;
        let trade_str = r#"{"e":"trade","E":1742583237873,"T":1742583237873,"s":"BTCUSDT","t":6126585299,"p":"84088.00","q":"0.002","X":"MARKET","m":false}"#;
        let order_trade_update_new = r#"{"e":"ORDER_TRADE_UPDATE","T":1742584483137,"E":1742584483137,"o":{"s":"BTCUSDT","c":"BG1gnPa6hrGgKVYZFOAvAu","S":"BUY","o":"LIMIT","f":"GTC","q":"1","p":"50000","ap":"0","sp":"0","x":"NEW","X":"NEW","i":4129466756,"l":"0","z":"0","L":"0","n":"0","N":"USDT","T":1742584483137,"t":0,"b":"50000","a":"0","m":false,"R":false,"wt":"CONTRACT_PRICE","ot":"LIMIT","ps":"BOTH","cp":false,"rp":"0","pP":false,"si":0,"ss":0,"V":"EXPIRE_MAKER","pm":"NONE","gtd":0}}"#;
        let trade_lite = r#" {"e":"TRADE_LITE","E":1742585119595,"T":1742585119595,"s":"BTCUSDT","q":"0.100","p":"0.00","m":false,"c":"KWsJ1UBaWM1C0IhvgdXxZb","S":"BUY","L":"84146.50","l":"0.100","t":310194316,"i":4129479931}"#;
        let order_trade_update_filled = r#"{"e":"ORDER_TRADE_UPDATE","T":1742585119595,"E":1742585119596,"o":{"s":"BTCUSDT","c":"KWsJ1UBaWM1C0IhvgdXxZb","S":"BUY","o":"MARKET","f":"GTC","q":"0.100","p":"0","ap":"84146.50000","sp":"0","x":"TRADE","X":"FILLED","i":4129479931,"l":"0.100","z":"0.100","L":"84146.50","n":"3.36586000","N":"USDT","T":1742585119595,"t":310194316,"b":"0","a":"0","m":false,"R":false,"wt":"CONTRACT_PRICE","ot":"MARKET","ps":"BOTH","cp":false,"rp":"0","pP":false,"si":0,"ss":0,"V":"EXPIRE_MAKER","pm":"NONE","gtd":0}}"#;
        let account_update = r#"{"e":"ACCOUNT_UPDATE","T":1742585119595,"E":1742585119596,"a":{"B":[{"a":"USDT","wb":"15237.42937172","cw":"15237.42937172","bc":"0"}],"P":[{"s":"BTCUSDT","pa":"0.100","ep":"84146.5","cr":"123.98240000","up":"0.40867555","mt":"cross","iw":"0","ps":"BOTH","ma":"USDT","bep":"84180.1586"}],"m":"ORDER"}}"#;

        let event_str_vec = vec![
            book_ticker_str,
            trade_str,
            order_trade_update_new,
            order_trade_update_filled,
            trade_lite,
            account_update,
        ];

        for s in event_str_vec {
            println!("src: {s}");
            let event = parse_event(s).unwrap();
            match event {
                WebsocketEvent::BookTicker(bookTicker) => {
                    println!("Parsed: {:?}", bookTicker);
                }
                WebsocketEvent::Trade(trade) => {
                    println!("Parsed: {:?}", trade);
                }
                WebsocketEvent::OrderTradeUpdate(order_trade_update) => {
                    println!("Parsed: {:?}", order_trade_update);
                }
                WebsocketEvent::TradeLite(trade_lite) => {
                    println!("Parsed: {:?}", trade_lite);
                }
                WebsocketEvent::AccountUpdate(account_update) => {
                    println!("Parsed: {:?}", account_update);
                }
                _ => {
                    panic!("Unexpected event type: {:?}", event);
                }
            }
        }
    }
}
