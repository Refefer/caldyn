use std::collections::BTreeMap;

/// Alias for simple one-arity functions
pub type Func = fn(f64) -> f64;

lazy_static!{
    static ref FUNCTIONS: BTreeMap<String, Func> = {
        let mut map = BTreeMap::<String, fn(f64) -> f64>::new();
        map.insert("sqrt".into(), f64::sqrt);
        map.insert("cbrt".into(), f64::cbrt);
        map.insert("sin".into(), f64::sin);
        map.insert("cos".into(), f64::cos);
        map.insert("tan".into(), f64::tan);
        map.insert("asin".into(), f64::asin);
        map.insert("acos".into(), f64::acos);
        map.insert("atan".into(), f64::atan);
        map.insert("sinh".into(), f64::sinh);
        map.insert("cosh".into(), f64::cosh);
        map.insert("tanh".into(), f64::tanh);
        map.insert("asinh".into(), f64::asinh);
        map.insert("acosh".into(), f64::acosh);
        map.insert("atanh".into(), f64::atanh);
        map.insert("floor".into(), f64::floor);
        map.insert("ceil".into(), f64::ceil);
        map.insert("abs".into(), f64::abs);
        map.insert("exp".into(), f64::exp);
        map.insert("ln".into(), f64::ln);
        map.insert("log2".into(), f64::log2);
        map.insert("log10".into(), f64::log10);
        map
    };
}

/// Provides a default accessor for FMaps using builtins
pub struct DefaultFMap;

pub static DEFAULT_FMAP: DefaultFMap = DefaultFMap;

/// Trait providing and describing a lookup map custom functions.
pub trait FMap {
    /// Indicates whether the FMap can provide a function for the
    /// given key
    fn is_function(&self, key: &str) -> bool;

    /// Returns a function definition for a given string if able
    fn get(&self, key: &str) -> Option<Func>;
}


impl FMap for DefaultFMap{
    fn is_function(&self, key: &str) -> bool {
        FUNCTIONS.contains_key(key)
    }

    fn get(&self, key: &str) -> Option<Func> {
        if let Some(&f) = FUNCTIONS.get(key) {
            Some(f)
        } else {
            None
        }
    }
}

/// Simple struct for having multiple layers of resolution
pub struct FMapSet(Vec<Box<FMap>>);

impl FMap for FMapSet {

    fn is_function(&self, key: &str) -> bool {
        for fm in self.0.iter() {
            if fm.is_function(key) {
                return true;
            }
        }
        return false
    }

    fn get(&self, key: &str) -> Option<Func> {
        for fm in self.0.iter() {
            if let Some(f) = fm.get(key) {
                return Some(f)
            }
        }
        None
    }

}
