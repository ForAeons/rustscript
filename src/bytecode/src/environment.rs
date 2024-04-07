use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{Symbol, Value, W};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Environment {
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub env: HashMap<Symbol, Value>,
}

impl Environment {
    /// Create a new frame with no parent, i.e. the root frame.
    pub fn new() -> Self {
        Environment {
            parent: None,
            env: HashMap::new(),
        }
    }

    /// Create the global environment.
    ///
    /// Constants are added to the global environment.
    /// - Logical constants: true, false
    /// - Math constants: PI, E
    /// - Environment constants: MAX_INT, MIN_INT, MAX_FLOAT, MIN_FLOAT, EPSILON
    ///
    /// Built in functions are added to the global environment.
    /// - Math functions: abs, ceil, floor, round, sqrt, sin, cos, tan, asin, acos, atan, atan2, ln, log2, log10, exp, pow
    /// - String functions: len
    /// - Type conversion functions: int_to_float, float_to_int, atoi, atoi
    /// - Comparison functions: min, max
    ///
    /// # Returns
    ///
    /// A wrapped reference to the global environment.
    pub fn new_global() -> Rc<RefCell<Self>> {
        let env = Environment::new_wrapped();

        // Global constants
        // Logical constants
        env.borrow_mut().set("true", true);
        env.borrow_mut().set("false", false);

        // Math constants
        env.borrow_mut().set("PI", std::f64::consts::PI);
        env.borrow_mut().set("E", std::f64::consts::E);

        //Environment constants
        env.borrow_mut().set("MAX_INT", std::i64::MAX);
        env.borrow_mut().set("MIN_INT", std::i64::MIN);
        env.borrow_mut().set("MAX_FLOAT", std::f64::MAX);
        env.borrow_mut().set("MIN_FLOAT", std::f64::MIN);
        env.borrow_mut().set("EPSILON", std::f64::EPSILON);

        // Built in functions
        // Math functions
        // env.borrow_mut().set("abs", );

        env
    }

    /// Create a wrapped frame with no parent, i.e. the root frame.
    pub fn new_wrapped() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment::new()))
    }

    /// Set the parent of the frame.
    pub fn set_parent(&mut self, parent: Rc<RefCell<Environment>>) {
        self.parent = Some(parent);
    }
}

impl Environment {
    /// Get a snapshot of the value of a symbol in the frame at the time of the call.
    pub fn get(&self, sym: &Symbol) -> Option<Value> {
        if let Some(val) = self.env.get(sym) {
            Some(val.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(sym)
        } else {
            None
        }
    }

    /// Set the value of a symbol in the frame.
    pub fn set(&mut self, sym: impl Into<Symbol>, val: impl Into<Value>) {
        self.env.insert(sym.into(), val.into());
    }
}

/// Environment should NOT be serialized. It is only used for runtime state.
/// This trait is pseudo-implemented so that we can add it to the operant stack.
/// Note we cannot implement Serialize for Rc<RefCell<Environment>> because it is not defined in this crate.
impl Serialize for W<Rc<RefCell<Environment>>> {
    fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        panic!("Environment should not be serialized");
    }
}

/// Environment should NOT be deserialized. It is only used for runtime state.
/// This trait is pseudo-implemented so that we can add it to the operant stack.
/// Note we cannot implement Deserialize for Rc<RefCell<Environment>> because it is not defined in this crate.
impl<'de> Deserialize<'de> for W<Rc<RefCell<Environment>>> {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        panic!("Environment should not be deserialized");
    }
}

/// Implement Clone trait to satisfy the requirements of Value enum.
impl Clone for W<Rc<RefCell<Environment>>> {
    fn clone(&self) -> Self {
        W(self.0.clone())
    }
}

/// Implement PartialEq trait to satisfy the requirements of Value enum.
impl PartialEq for W<Rc<RefCell<Environment>>> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Implement Debug trait to satisfy the requirements of Value enum.
impl Debug for W<Rc<RefCell<Environment>>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame() {
        let env = Environment::new_wrapped();
        env.borrow_mut().set("x", 42);
        assert_eq!(env.borrow().get(&"x".to_string()), Some(Value::Int(42)));
    }

    #[test]
    fn test_frame_with_parent() {
        let parent_env = Environment::new_wrapped();
        parent_env.borrow_mut().set("x", 42);
        let child_env = Environment::new_wrapped();
        child_env.borrow_mut().set_parent(parent_env);
        child_env.borrow_mut().set("y", 43);
        assert_eq!(
            child_env.borrow().get(&"x".to_string()),
            Some(Value::Int(42))
        );
        assert_eq!(
            child_env.borrow().get(&"y".to_string()),
            Some(Value::Int(43))
        );
    }
}
