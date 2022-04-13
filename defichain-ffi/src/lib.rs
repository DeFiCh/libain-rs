pub mod types;
pub mod storage;
pub mod error;

#[link(name = "boost_system")]
#[link(name = "boost_filesystem")]
extern "C" {
    fn print_time();
}