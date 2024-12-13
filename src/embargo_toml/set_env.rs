use crate::error::EmbargoError;
use std::env;

use super::const_values::ENV_VAR_NAME;

/// This attempts to set the environment variable for EMBARGO_HOME depending on the platform
/// Right now it's only set up to work on Mac OSX, so TODO
pub fn try_set_env() -> Result<String, EmbargoError> {
    let var = "~/Library/Application Support/Embargo";
    env::set_var(ENV_VAR_NAME, var);
    Ok(String::from(var))
}