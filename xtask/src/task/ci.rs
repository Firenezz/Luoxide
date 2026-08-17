use super::test;
use crate::Result;

pub fn run(args: &[String]) -> Result<()> {
    // Write `.snap.new` on mismatch so CI can upload them; no files means
    // snapshots passed (or the failure was unrelated).
    std::env::set_var("INSTA_UPDATE", "new");
    test::run(args)
}
