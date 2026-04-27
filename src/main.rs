pub mod app;
pub mod event;
pub mod ui;

#[tokio::main]
async fn main() {
    let terminal = ratatui::init();
    let _result = crate::app::App::new().run(terminal).await;
    println!("Hello, world!");
    ratatui::restore();
}
