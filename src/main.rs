use explore_bzl::explore_bzl;

#[tokio::main]
async fn main() {
  let terminal = ratatui::init();
  let _result = explore_bzl::run(terminal).await;
  ratatui::restore();
}
