use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref QUESTION_MAP: Mutex<HashMap<i64, Vec<String>>> = Mutex::new(HashMap::new());
}

pub fn add_questions(user_id: i64, new_questions: Vec<String>) {
    let mut question_map = QUESTION_MAP.lock().unwrap();
    question_map.entry(user_id).or_insert(Vec::new()).extend(new_questions);
}

pub fn remove_last_question(user_id: i64) {
    let mut question_map = QUESTION_MAP.lock().unwrap();
    if let Some(questions) = question_map.get_mut(&user_id) {
        questions.pop();
    }
}
pub fn get_last_question(user_id: i64) -> Option<String> {
    let question_map = QUESTION_MAP.lock().unwrap();
    question_map
        .get(&user_id)
        .and_then(|questions| questions.last().cloned())
}

pub fn remove_all_questions(user_id: i64) {
    let mut question_map = QUESTION_MAP.lock().unwrap();
    question_map.remove(&user_id);
}
