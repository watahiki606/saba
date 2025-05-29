#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;
use noli::*;
use saba_core::browser::Browser;
use saba_core::http::HttpResponse;
use ui_wasabi::app::WasabiUI;

static TEST_HTTP_RESPONSE: &str = r#"
HTTP/1.1 200 OK
Data: xx xx xx

<html>
<head></head>
<body>
  <h1 id="title">H1 title</h1>
  <h2 class="class">H2 title</h2>
  <p>Test text.</p>
  <p>
  <a href= "example.com">Link1</a>
  <a href= "example.com">Link2</a>
  </p>
</body>
</html>
"#;

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
