use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::currency::Currency;

/// Balance is the account balance resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub balance: HashMap<Currency, f64>,
}
