#![no_std]
#![no_main]

extern crate alloc;

use alloc::rc::Rc;
use core::cell::RefCell;
use noli::prelude::*;
use saba_core::browser::Browser;
use ui_wasabi::app::WasabiUI;

fn main() -> u64 {
    // Browser 構造体を初期化
    let browser = Browser::new();

    // WasabiUI 構造体を初期化
    let ui = Rc::new(RefCell::new(WasabiUI::new(browser)));

    // アプリの実行を開始
    match ui.borrow_mut().start() {
        Ok(_) => {}
        Err(e) => {
            println!("browser fails to start {:?}", e);
            return 1;
        }
    };

    0
}

entry_point!(main);
