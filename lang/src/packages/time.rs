use std::collections::HashMap;
use std::time;
use chrono::prelude::{DateTime, Local};

use crate::ast::AST;
use crate::eval::eval;

// deprecated
pub fn now(_: Vec<AST>,  _: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    println!("Warning: time.now() is deprecated. Use time.now_unix() instead");

    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?;

    Ok((AST::Integer(now.as_secs() as i64), AST::Null))
}

pub fn now_unix(_: Vec<AST>,  _: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?;

    Ok((AST::Integer(now.as_secs() as i64), AST::Null))
}

pub fn now_utc(_: Vec<AST>,  _: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let now = time::SystemTime::now();
    let datetime: DateTime<chrono::Utc> = now.into();

    Ok((AST::String(format!("{}", datetime.format("%c"))), AST::Null))
}

pub fn now_local(_: Vec<AST>,  _: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let now = time::SystemTime::now();
    let datetime: DateTime<Local> = now.into();

    Ok((AST::String(format!("{}", datetime.format("%c"))), AST::Null))
}

pub fn to_iso_8601(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let time = match eval(args[0].clone(), context) {
        Ok(AST::Integer(time)) => time,
        Ok(AST::Float(time)) => time as i64,
        
        Ok(_) => return Err("time.to_iso_8601() expects a number".to_string()),
        Err(e) => return Err(e),
    };

    let time = time::UNIX_EPOCH + time::Duration::from_secs(time as u64);
    let time: DateTime<Local> = time.into();
    let time = time.format("%+").to_string();


    Ok((AST::String(time), AST::Null))
}

pub fn to_rfc_2822(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let time = match eval(args[0].clone(), context) {
        Ok(AST::Integer(time)) => time,
        Ok(AST::Float(time)) => time as i64,
        
        Ok(_) => return Err("time.to_rfc_2822() expects a number".to_string()),
        Err(e) => return Err(e),
    };

    let time = time::UNIX_EPOCH + time::Duration::from_secs(time as u64);
    let time: DateTime<Local> = time.into();
    let time = time.to_rfc2822();

    Ok((AST::String(time), AST::Null))
}

pub fn to_local_date_time(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let time = match eval(args[0].clone(), context) {
        Ok(AST::Integer(time)) => time,
        Ok(AST::Float(time)) => time as i64,
        
        Ok(_) => return Err("time.to_local_date_time() expects a number".to_string()),
        Err(e) => return Err(e),
    };

    let time = time::UNIX_EPOCH + time::Duration::from_secs(time as u64);
    let time: DateTime<Local> = time.into();
    let time = time.format("%c").to_string();


    Ok((AST::String(time), AST::Null))
}

pub fn to_utc_date_time(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let time = match eval(args[0].clone(), context) {
        Ok(AST::Integer(time)) => time,
        Ok(AST::Float(time)) => time as i64,
        
        Ok(_) => return Err("time.to_utc_date_time() expects a number".to_string()),
        Err(e) => return Err(e),
    };

    let time = time::UNIX_EPOCH + time::Duration::from_secs(time as u64);
    let time: DateTime<chrono::Utc> = time.into();
    let time = time.format("%c").to_string();

    Ok((AST::String(time), AST::Null))
}

pub fn get_object() -> HashMap<String, AST> {
    let mut object = HashMap::new();

    object.insert(
        "now".to_string(),
        AST::InternalFunction { 
            name: "now".to_string(),
            args: vec![],
            call_fn: now
        }
    );

    object.insert(
        "now_unix".to_string(),
        AST::InternalFunction { 
            name: "now_unix".to_string(),
            args: vec![],
            call_fn: now_unix
        }
    );

    object.insert(
        "now_utc".to_string(),
        AST::InternalFunction { 
            name: "now_utc".to_string(),
            args: vec![],
            call_fn: now_utc
        }
    );

    object.insert(
        "now_local".to_string(),
        AST::InternalFunction { 
            name: "now_local".to_string(),
            args: vec![],
            call_fn: now_local
        }
    );

    object.insert(
        "to_iso_8601".to_string(),
        AST::InternalFunction { 
            name: "to_iso_8601".to_string(), 
            args: vec!["unix".to_string()], 
            call_fn: to_iso_8601 
        }
    );

    object.insert(
        "to_rfc_2822".to_string(),
        AST::InternalFunction { 
            name: "to_rfc_2822".to_string(), 
            args: vec!["unix".to_string()], 
            call_fn: to_rfc_2822
        }
    );

    object.insert(
        "to_local_date_time".to_string(),
        AST::InternalFunction { 
            name: "to_local_date_time".to_string(), 
            args: vec!["unix".to_string()], 
            call_fn: to_local_date_time 
        }
    );

    object.insert(
        "to_utc_date_time".to_string(),
        AST::InternalFunction { 
            name: "to_utc_date_time".to_string(), 
            args: vec!["unix".to_string()], 
            call_fn: to_utc_date_time 
        }
    );

    return object;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_time_package() {
        let time = get_object();
        assert_eq!(time.len(), 8);
        assert_eq!(time.contains_key("now"), true);
    }

    #[test]
    fn get_current_time() {
        let time = now(vec![], &mut HashMap::new()).unwrap().0;
        
        assert_eq!(
            time,
            AST::Integer(
                time::SystemTime::now()
                    .duration_since(time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
            ),
        )
    }
}