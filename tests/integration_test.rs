// Integration test for SigmaOS-betteroffice-docs
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use betteroffice_docs as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
