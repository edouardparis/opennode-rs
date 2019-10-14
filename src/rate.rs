use std::collections::HashMap;

use crate::currency::Currency;

/// Rates is the rates resource.
pub type Rates = HashMap<String, HashMap<Currency, f64>>;
