// use svm_rs_builds::ALL_SOLC_VERSIONS;
use svm_rs_builds::ALL_SOLC_VERSIONS;

fn main() {
    let mut all_versions = ALL_SOLC_VERSIONS.clone();
    all_versions.sort(); // ascending ordering
    println!("{:#?}", all_versions);
}
