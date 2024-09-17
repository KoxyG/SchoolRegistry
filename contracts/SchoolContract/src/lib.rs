#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, Env, IntoVal, Symbol, TryFromVal, Val, Vec};


const STUDENT: Symbol = symbol_short!("STUDENT");
const TEACHER: Symbol = symbol_short!("TEACHER");

#[contract]
pub struct SchoolContract;

#[contracttype]

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
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
        let mut students = env.storage().instance().get(&STUDENT).unwrap_or_else(|| Vec::new(&env));

        let new_student = Student { id, name, age, class, height };
        
        students.push_back(new_student.clone());
        
        env.storage().instance().set(&STUDENT, &students);
        env.storage().instance().extend_ttl(100, 100);
        new_student
    }


    pub fn add_teacher(env: Env, id: u32, name: Symbol, age: u32, subject: Symbol, salary: u32) -> Teacher {
        let mut teachers = env.storage().instance().get(&TEACHER).unwrap_or_else(|| Vec::new(&env));
       
        let new_teacher = Teacher { id, name, age, subject, salary };
        
        teachers.push_back(new_teacher.clone());

        env.storage().instance().set(&TEACHER, &teachers);
        env.storage().instance().extend_ttl(100, 100);
        new_teacher
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