// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-keyboard-navigation: keyboard navigation component model.
//!
//! Real implementation: navigations, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Navigation {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyboardNavigationError {
    EmptyNavigationName,
    DuplicateNavigation(String),
    EmptyEventSubject,
}

impl fmt::Display for KeyboardNavigationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNavigationName => write!(f, "navigation name must not be empty"),
            Self::DuplicateNavigation(id) => write!(f, "duplicate navigation {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for KeyboardNavigationError {}

pub struct KeyboardNavigation {
    pub name: String,
    navigations: BTreeMap<String, Navigation>,
    events: BTreeMap<String, Event>,
}

impl KeyboardNavigation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            navigations: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_navigation(&mut self, item: Navigation) -> Result<(), KeyboardNavigationError> {
        if item.name.trim().is_empty() {
            return Err(KeyboardNavigationError::EmptyNavigationName);
        }
        let id = item.id.clone();
        if self.navigations.contains_key(&id) {
            return Err(KeyboardNavigationError::DuplicateNavigation(id.clone()));
        }
        self.navigations.insert(id, item);
        Ok(())
    }

    pub fn navigation_count(&self) -> usize {
        self.navigations.len()
    }

    pub fn get_navigation(&self, id: &str) -> Option<&Navigation> {
        self.navigations.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), KeyboardNavigationError> {
        if event.subject.trim().is_empty() {
            return Err(KeyboardNavigationError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_navigation(id: &str, name: &str) -> Navigation {
        Navigation {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = KeyboardNavigation::new("KeyboardNavigation");
        assert_eq!(m.name, "KeyboardNavigation");
        assert_eq!(m.navigation_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_navigation() {
        let mut m = KeyboardNavigation::new("KeyboardNavigation");
        m.add_navigation(sample_navigation("a1", "Alpha")).unwrap();
        assert_eq!(m.navigation_count(), 1);
        assert!(m.get_navigation("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = KeyboardNavigation::new("KeyboardNavigation");
        assert!(m.add_navigation(sample_navigation("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = KeyboardNavigation::new("KeyboardNavigation");
        m.add_navigation(sample_navigation("x", "One")).unwrap();
        assert!(m.add_navigation(sample_navigation("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = KeyboardNavigation::new("KeyboardNavigation");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
