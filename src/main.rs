mod maching_engine;
use maching_engine::orderbook::{Order, Orderbook,BidOrAsk};

fn main() {
    // let mut limit = Limit::new(65.3);
    let buy_order_from_alic = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Ask, 2.45);
    let mut orderbook = Orderbook::new();
    // limit.add_order(buy_order);
    orderbook.add_order(4.4, buy_order_from_alic);
    orderbook.add_order(4.4, buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask,6.5);
    orderbook.add_order(20.0, sell_order);
    println!("{:?} ",orderbook);
}
