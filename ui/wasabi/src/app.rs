use crate::alloc::string::ToString;
use alloc::format;
use alloc::rc::Rc;
use core::cell::RefCell;
use noli::error::Result as OSResult;
use noli::window::StringSize;
use noli::window::Window;
use saba_core::browser::Browser;
use saba_core::constants::WHITE;
use saba_core::constants::WINDOW_HEIGHT;
use saba_core::constants::WINDOW_INIT_X_POS;
use saba_core::constants::WINDOW_INIT_Y_POS;
use saba_core::constants::WINDOW_WIDTH;
use saba_core::constants::*;
use saba_core::error::Error;

#[derive(Debug)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
}

impl WasabiUI {
    pub fn new(browser: Rc<RefCell<Browser>>) -> Self {
        Self {
            browser,
            window: Window::new(
                "saba".to_string(),
                WHITE,
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .unwrap(),
        }
    }

    fn setup_toolbar(&mut self) -> OSResult<()> {
        // ツールバーの背景の資格を描画
        self.window
            .fill_rect(LIGHTGREY, 0, 0, WINDOW_WIDTH, ADDRESSBAR_HEIGHT)?;

        // ツールバーとコンテンツエリアの境界線を描画
        self.window
            .draw_line(GREY, 0, TOOLBAR_HEIGHT, WINDOW_WIDTH - 1, TOOLBAR_HEIGHT)?;
        self.window.draw_line(
            DARKGREY,
            0,
            TOOLBAR_HEIGHT + 1,
            WINDOW_WIDTH - 1,
            TOOLBAR_HEIGHT + 1,
        )?;

        // アドレスバーの横に"Address"というテキストを描画
        self.window.draw_string(
            BLACK,
            5,
            5,
            "Address",
            StringSize::Medium,
            /*underline=*/ false,
        )?;

        // アドレスバーの資格を描画
        self.window
            .fill_rect(WHITE, 70, 2, WINDOW_WIDTH - 74, 2 + ADDRESSBAR_HEIGHT)?;

        // アドレスバーの影の線を描画
        self.window.draw_line(GREY, 70, 2, WINDOW_WIDTH - 4, 2)?;
        self.window
            .draw_line(GREY, 70, 2, 70, 2 + ADDRESSBAR_HEIGHT)?;
        self.window.draw_line(BLACK, 71, 3, WINDOW_WIDTH - 5, 3)?;

        self.window.draw_line(GREY, 71, 3, 71, 1 + WINDOW_WIDTH)?;

        Ok(())
    }

    fn setup(&mut self) -> Result<(), Error> {
        if let Err(error) = self.setup_toolbar() {
            // OsResult と Result が持つ Error 型は異なるので変換する
            return Err(Error::InvalidUI(format!(
                "failed to initilalize a toolbar with error: {:#?}",
                error
            )));
        }
        // 画面を更新する
        self.window.flush();

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.setup()?;

        self.run_app()?;

        Ok(())
    }

    fn run_app(&mut self) -> Result<(), Error> {
        // 後ほど実装する
        Ok(())
    }
}
