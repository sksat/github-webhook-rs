#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CaseConvention {
    Lower,
    Upper,
    Snake,
    Pascal,
    ScreamingSnake,
}
use CaseConvention::*;

use crate::ir::RenameRule;

/// expect input follows one of above case
pub fn detect_case(s: &str) -> CaseConvention {
    if s.starts_with(char::is_uppercase) {
        if s.contains('_') {
            ScreamingSnake
        } else if s.chars().all(char::is_uppercase) {
            Upper
        } else {
            Pascal
        }
    } else if s.contains('_') {
        Snake
    } else {
        Lower
    }
}
impl CaseConvention {
    pub fn cast(&mut self, other: Self) -> Option<()> {
        if self == &other {
            return Some(());
        }
        match (&self, &other) {
            (Lower, Snake) | (Upper, ScreamingSnake) => {
                *self = other;
            }
            (Snake, Lower) | (ScreamingSnake, Upper) => {
                // do nothing
            }
            _ => None?,
        }
        Some(())
    }
    pub fn into_rename_rule(self) -> RenameRule {
        match self {
            Lower | Snake => RenameRule::SnakeCase,
            Pascal => RenameRule::PascalCase,
            Upper | ScreamingSnake => RenameRule::ScreamingSnakeCase,
        }
    }
}
