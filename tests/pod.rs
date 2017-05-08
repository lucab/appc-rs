extern crate appc;
extern crate serde_json;

use std::{io, fs};

#[test]
fn test_pod_manifest_example() {
    let f = fs::File::open("tests/fixtures/example-pod-manifest.json").expect("Missing fixture");
    let bufrd = io::BufReader::new(f);
    let _manif: appc::schema::PodManifest = serde_json::from_reader(bufrd).unwrap();
}

#[test]
fn test_pod_dup_volumes() {
    let f = fs::File::open("tests/fixtures/pod-dup-volumes.json").expect("Missing fixture");
    let bufrd = io::BufReader::new(f);
    let r: Result<appc::schema::PodManifest, serde_json::Error> = serde_json::from_reader(bufrd);
    assert!(r.is_err())
}
