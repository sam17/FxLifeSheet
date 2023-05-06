use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref QUESTION_MAP: Mutex<HashMap<i64, Vec<String>>> = Mutex::new(HashMap::new());
    static ref CURRENT_QUESTION: Mutex<HashMap<i64, Option<String>>> = Mutex::new(HashMap::new());
}

pub fn add_questions(user_id: i64, new_questions: Vec<String>) {
    let mut question_map = QUESTION_MAP.lock().unwrap();
    question_map.entry(user_id).or_insert(Vec::new()).extend(new_questions);
}
pub fn get_first_question(user_id: i64) -> Option<String> {
    let mut question_map = QUESTION_MAP.lock().unwrap();
    let questions = question_map.get_mut(&user_id)?;
    let first_question = Some(questions.remove(0));
    CURRENT_QUESTION.lock().unwrap().insert(user_id, first_question.clone());
    first_question
}

pub fn remove_all_questions(user_id: i64) {
    let mut question_map = QUESTION_MAP.lock().unwrap();
    question_map.remove(&user_id);
}


pub fn get_all_questions(user_id: i64) -> Option<Vec<String>> {
    let question_map = QUESTION_MAP.lock().unwrap();
    question_map.get(&user_id).cloned()
}

pub fn is_question_queue_empty(user_id: i64) -> bool {
    let question_map = QUESTION_MAP.lock().unwrap();
    question_map.get(&user_id).map_or(true, |questions| questions.is_empty())
}

pub fn get_current_question(user_id: i64) -> Option<String> {
    let current_question = CURRENT_QUESTION.lock().unwrap();
    current_question.get(&user_id).and_then(|x| x.clone())
}