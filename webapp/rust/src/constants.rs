pub const DEFAULT_PAYMENT_SERVICE_URL: &str = "http://127.0.0.1:5555";
pub const DEFAULT_SHIPMENT_SERVICE_URL: &str = "http://127.0.0.1:7000";

pub const ITEM_STATUS_ON_SALE: &str = "on_sale";
pub const ITEM_STATUS_TRADING: &str = "trading";
pub const ITEM_STATUS_SOLD_OUT: &str = "sold_out";
pub const ITEM_STATUS_STOP: &str = "stop";
pub const ITEM_STATUS_CANCEL: &str = "cancel";
pub const TRANSACTION_EVIDENCE_STATUS_WAIT_SHIPPING: &str = "wait_shipping";
pub const TRANSACTION_EVIDENCE_STATUS_WAIT_DONE: &str = "wait_done";
pub const TRANSACTION_EVIDENCE_STATUS_DONE: &str = "done";

pub const SHIPPING_STATUS_INITIAL: &str = "initial";
pub const SHIPPING_STATUS_WAIT_PICKUP: &str = "wait_pickup";
pub const SHIPPING_STATUS_SHIPPING: &str = "shipping";
pub const SHIPPING_STATUS_DONE: &str = "done";

pub const ISUCARI_API_TOKEN: &str = "Bearer 75ugk2m37a750fwir5xr-22l6h4wmue1bwrubzwd0";

pub const PAYMENT_SERVICE_ISUCARI_API_KEY: &str = "a15400e46c83635eb181-946abb51ff26a868317c";
pub const PAYMENT_SERVICE_ISUCARI_SHOP_ID: &str = "11";

pub const ITEMS_PER_PAGE: usize = 48;
pub const TRANSACTIONS_PER_PAGE: usize = 10;
