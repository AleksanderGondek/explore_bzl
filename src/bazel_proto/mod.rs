// No influence over structure of generated code
#![allow(clippy::all)]
#![allow(clippy::pedantic)]
pub mod blaze_query {
  include!(concat!(env!("OUT_DIR"), "/blaze_query.rs"));
}
pub mod stardoc_output {
  include!(concat!(env!("OUT_DIR"), "/stardoc_output.rs"));
}
