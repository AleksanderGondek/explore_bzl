use explore_bzl::explore_bzl;

#[tokio::main(worker_threads = 4)]
async fn main() {
  let terminal = ratatui::init();
  let _result = explore_bzl::run(terminal).await;
  ratatui::restore();
}
