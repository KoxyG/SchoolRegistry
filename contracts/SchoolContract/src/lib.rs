#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, Env, IntoVal, Symbol, TryFromVal, Val, Vec};


const STUDENT: Symbol = symbol_short!("STUDENT");
const TEACHER: Symbol = symbol_short!("TEACHER");

#[contract]
pub struct SchoolContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct Student {
    id: u32,
    name: Symbol,
    age: u32,
    class: Symbol,
    height: u32,
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct Teacher {
    id: u32,
    name: Symbol,
    age: u32,
    subject: Symbol,
    salary: u32,
}




#[contractimpl]
impl SchoolContract {
    pub fn add_student(env: Env, id: u32, name: Symbol, age: u32, class: Symbol, height: u32) -> Student {
        let mut student = env.storage().instance().get(&STUDENT).unwrap();

        student = Student { id, name, age, class, height };
       

       
        env.storage().instance().set(&STUDENT, &student);
        env.storage().instance().extend_ttl(100, 100);

        student
    }

    pub fn add_teacher(env: Env, id: u32, name: Symbol, age: u32, subject: Symbol, salary: u32) -> Teacher {
        let mut teacher = env.storage().instance().get(&TEACHER).unwrap();

        teacher = Teacher { id, name, age, subject, salary };
        env.storage().instance().set(&TEACHER, &teacher);
        env.storage().instance().extend_ttl(100, 100);

        teacher
    }


    pub fn get_student(env: Env) -> Student {
        env.storage().instance().get(&STUDENT).unwrap()
    }

    pub fn get_teacher(env: Env) -> Teacher {
        env.storage().instance().get(&TEACHER).unwrap()
    }

    pub fn get_a_student(env: Env, id: u32) {
       
    }
    

    
}

mod test;