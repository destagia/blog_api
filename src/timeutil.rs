/*
  Time format =
    YYYY-MM-DDTHH24:MI:SS (postgresql)
    %y-%m-%l-%T%k:%M:%S   (rust)
*/

use time;
use time::{Tm, ParseError};

fn str_to_tm(str: &String) -> Option<Tm> {
    return match time::strptime(str, &"%y-%m-%l-%T%k:%M:%S".to_string()) {
        Ok(tm) => Some(tm),
        Err(_) => None
    }
}
