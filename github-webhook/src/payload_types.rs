#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::large_enum_variant)]

use serde::Deserialize;
use serde_json::Value;

use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/types.rs"));
