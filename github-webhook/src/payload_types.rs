#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WebhookEvents {}

#[derive(Debug, Deserialize)]
pub struct PullRequestReviewRequestRemovedEvent {}

#[derive(Debug, Deserialize)]
pub struct PullRequestReviewRequestedEvent {}

include!(concat!(env!("OUT_DIR"), "/types.rs"));
