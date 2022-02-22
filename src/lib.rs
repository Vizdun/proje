#![allow(clippy::wildcard_imports)]

use num_bigint::BigUint;
use seed::{app::orders::Orders, prelude::*, *};

fn init(_: Url, o: &mut impl Orders<Msg>) -> Model {
    o.stream(streams::interval(1000, || Msg::Increment));
    Model {
        emc: BigUint::from(0 as u32),
        flowers: BigUint::from(1 as u32),
        watches: BigUint::from(0 as u32),
    }
}

struct Model {
    emc: BigUint,
    flowers: BigUint,
    watches: BigUint,
}

#[derive(Copy, Clone)]
enum Msg {
    Increment,
    BuyFlower,
    BuyWatch,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => {
            let base = 680 as u32 * model.flowers.clone();
            let add = base.clone() * model.watches.clone();
            model.emc += base + add
        }
        Msg::BuyFlower => {
            model.flowers += 1 as u32;
            model.emc -= 15_548_153 as u32
        }
        Msg::BuyWatch => {
            model.watches += 1 as u32;
            model.emc -= 560_256 as u32
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        h1![C!["text-center"], format!("{}", model.emc)],
        C!["container position-absolute top-50 start-50 translate-middle"],
        [
            (
                "Power Flowers",
                15_548_153,
                Msg::BuyFlower,
                model.flowers.clone()
            ),
            (
                "Watches of Flowing Time",
                560_256,
                Msg::BuyWatch,
                model.watches.clone()
            )
        ]
        .map(|(s, p, b, c)| div![
            C!["row align-items-center p-3"],
            div![
                C!["col"],
                p![C!["text-right m-0 fs-6"], format!("{}", c)],
                p![C!["m-0 fs-5"], s],
                p![C!["text-right m-0 fs-6"], format!("{} EMC", p)]
            ],
            div![
                C!["col-3"],
                if model.emc > BigUint::from(p as u32) {
                    button![
                        C!["btn btn-primary float-end"],
                        ev(Ev::Click, move |_| b),
                        "Buy"
                    ]
                } else {
                    button![C!["btn btn-secondary float-end"], "Buy"]
                },
            ],
        ],)
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    let app = App::start("app", init, update, view);
}
