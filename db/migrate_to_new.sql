-- Make sure category table is populated with all categories from old_questions table
INSERT INTO new_questions
(
    key,
    question,
    answer_type,
    category,
    max,
    min,
    show,
    display_name,
    is_positive,
    cadence,
    graph_type
)
SELECT
    oq.key,
    oq.question,
    oq.question_type AS answer_type,
    c.id AS category,
    oq.max_value AS max,
    oq.min_value AS min,
    oq.is_visible_in_visualizer AS show,
    oq.display_name,
    oq.is_positive,
    oq.cadence,
    oq.graph_type
FROM
    old_questions oq
    JOIN
    category c ON oq.category = c.name;