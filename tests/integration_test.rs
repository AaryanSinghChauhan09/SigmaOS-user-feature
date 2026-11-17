// Integration test for SigmaOS-user-feature
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use user_feature as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
