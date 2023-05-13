use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use models::models::questions::viz_questions::Question;

lazy_static! {
    static ref QUESTION_MAP: Mutex<HashMap<i64, Vec<Question>>> = Mutex::new(HashMap::new());
    static ref CURRENT_QUESTION: Mutex<HashMap<i64, Option<Question>>> = Mutex::new(HashMap::new());
}

pub fn add_questions(user_id: i64, new_questions: Vec<Question>) {
    let mut question_map = QUESTION_MAP.lock().unwrap();
    question_map.entry(user_id).or_insert(Vec::new()).extend(new_questions);
}

pub fn add_questions_to_front(user_id: i64, new_questions: Vec<Question>) {
    let mut question_map = QUESTION_MAP.lock().unwrap();
    question_map.entry(user_id).or_insert(Vec::new()).splice(0..0, new_questions);
}

pub fn set_current_question_nulled(user_id: i64) {
    let mut current_question = CURRENT_QUESTION.lock().unwrap();
    current_question.insert(user_id, None);
}

pub fn get_first_question(user_id: i64) -> Option<Question> {
    let mut question_map = QUESTION_MAP.lock().unwrap();
    let questions = question_map.get_mut(&user_id)?;

    if questions.is_empty() {
        None
    } else {
        let first_question = Some(questions.remove(0));
        CURRENT_QUESTION.lock().unwrap().insert(user_id, first_question.clone());
        first_question
    }
}

pub fn remove_all_questions(user_id: i64) {
    let mut question_map = QUESTION_MAP.lock().unwrap();
    question_map.remove(&user_id);
}


pub fn get_all_questions(user_id: i64) -> Option<Vec<Question>> {
    let question_map = QUESTION_MAP.lock().unwrap();
    question_map.get(&user_id).cloned()
}

pub fn is_question_queue_empty(user_id: i64) -> bool {
    let question_map = QUESTION_MAP.lock().unwrap();
    question_map.get(&user_id).map_or(true, |questions| questions.is_empty())
}

pub fn get_current_question(user_id: i64) -> Option<Question> {
    let current_question = CURRENT_QUESTION.lock().unwrap();
    current_question.get(&user_id).and_then(|x| x.clone())
}