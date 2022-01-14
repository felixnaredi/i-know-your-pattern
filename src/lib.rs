#[macro_use]
extern crate lazy_static;

mod pattern_tracker;
mod utils;

use std::sync::Mutex;

use pattern_tracker::PatternTracker;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref PATTERN_TRACKER: Mutex<PatternTracker> = Mutex::new(PatternTracker::new());
}

#[derive(PartialEq)]
#[wasm_bindgen]
pub enum Input
{
    Black,
    White,
}

impl Into<bool> for Input
{
    fn into(self) -> bool
    {
        self == Input::White
    }
}

impl Into<Input> for bool
{
    fn into(self) -> Input
    {
        if self {
            Input::White
        } else {
            Input::Black
        }
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet()
{
    alert(&format!("i know you pattern - {:?}", *PATTERN_TRACKER));
}

#[wasm_bindgen]
pub fn push_black_input()
{
    PATTERN_TRACKER.lock().unwrap().push(Input::Black.into());
}

#[wasm_bindgen]
pub fn push_white_input()
{
    PATTERN_TRACKER.lock().unwrap().push(Input::White.into());
}

#[wasm_bindgen]
pub fn predict_next_input() -> Option<Input>
{
    PATTERN_TRACKER
        .lock()
        .unwrap()
        .predict_next()
        .map(bool::into)
}
