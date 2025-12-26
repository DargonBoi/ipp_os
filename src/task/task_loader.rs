use alloc::{string::String, vec::Vec};

use crate::{error::MyError, println};

use super::spawner::SPAWNER;

pub async fn load_task(input_str: String) -> Result<(), MyError> {
    match parse_string(input_str).await.get(0).expect("Invalid Argument").as_str() {
        "example_task" => SPAWNER.lock().add(example_task()),
        _ => return Err(MyError::InvalidFuture),
    };

    Ok(())
}


pub async fn parse_string(input_str: String) -> Vec<String> {
    let mut to_return: Vec<String> = Vec::new();
    for str in input_str.split_whitespace().map(|s| String::from(s)){
        to_return.push(str);
    };
    to_return
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}