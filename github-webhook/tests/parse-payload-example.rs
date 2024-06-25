//use std::assert_matches::assert_matches;
use std::env;

use github_webhook::payload_types::Schema;

fn download_example(endpoint: &str, kind: &str, payload: &str) -> String {
    let repo = "octokit/webhooks";
    let branch = "main";
    let payload = payload.to_string() + ".payload.json";
    let url = format!("https://raw.githubusercontent.com/{repo}/{branch}/payload-examples/{endpoint}/{kind}/{payload}");

    let out_dir = env!("OUT_DIR");
    let payload = format!("{out_dir}/{payload}");

    let response = minreq::get(&url).send().unwrap();
    if response.status_code / 200 != 1 {
        // failure
        panic!("Could not download {url}");
    }
    let body = response.as_str().unwrap();
    std::fs::write(&payload, body).unwrap();

    payload
}

#[test]
fn branch_protection_rule() {
    let event = vec!["created", "deleted", "edited"];
    for e in event {
        let payload = download_example("api.github.com", "branch_protection_rule", e);
        let payload = std::fs::read_to_string(payload).unwrap();

        let payload: Schema = serde_json::from_str(&payload).unwrap();
        dbg!(&payload);

        assert!(matches!(payload, Schema::BranchProtectionRuleEvent(_)));
    }
}
