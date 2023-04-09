interface QuestionData {
    key: string;
    max_value: number;
    min_value: number;
    question: string;
    question_type: string;
    buttons: string;
    is_positive: boolean;
    is_reverse: boolean;
    graph_type: string;
    display_name: string;
    cadence: string;
}

export default QuestionData;
