// SPDX-FileCopyrightText: The nonicle authors
// SPDX-License-Identifier: MPL-2.0

use super::*;

impl CanonicalOrd for String {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl IsCanonical for String {
    fn is_canonical(&self) -> bool {
        self.chars().all(char::is_lowercase)
    }
}

impl Canonicalize for String {
    fn canonicalize(&mut self) {
        let mut canonicalized = self.to_lowercase();
        std::mem::swap(self, &mut canonicalized);
    }
}

#[test]
fn canonicalize_vec() {
    assert_eq!(
        Canonical::tie(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
        vec![
            "B".to_string(),
            "A".to_string(),
            "c".to_string(),
            "a".to_string(),
            "C".to_string(),
            "b".to_string(),
            "c".to_string(),
            "b".to_string(),
            "a".to_string(),
            "A".to_string(),
            "B".to_string(),
        ]
        .canonicalize_into()
    );
}
