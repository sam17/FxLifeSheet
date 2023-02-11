interface QuestionData {
    key: string;
    max_value: number;
    min_value: number;
    question: string;
    question_type: string;
    buttons: string;
    is_positive: boolean;
    is_reverse: boolean;
    display_name: string;
}

export default QuestionData;
