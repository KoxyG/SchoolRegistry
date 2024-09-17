#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Env};

#[test]
#[test]
fn test_add_and_get_student() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SchoolContract);
    let client = SchoolContractClient::new(&env, &contract_id);

    // Add a new student
    let new_student = client.add_student(
        &1u32,
        &symbol_short!("Alice"),
        &10u32,
        &symbol_short!("SS1"),
        &100u32,
    );

    // Assert that the student added matches the expected details
    assert_eq!(
        new_student,
        Student {
            id: 1,
            name: symbol_short!("Alice"),
            age: 10,
            class: symbol_short!("SS1"),
            height: 100,
        }
    );

    // Retrieve all students from storage and assert correctness
    let stored_students = client.get_students();
    assert_eq!(stored_students.len(), 1);
    assert_eq!(stored_students.get(0).unwrap(), new_student);

    
}
