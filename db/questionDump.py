import json
import psycopg2
import sys
import requests

# Define the class for validating the data structure


class Question:
    def __init__(self, key, question, type, maxValue, minValue, isVisibleInVisualizer, buttons, category, displayName, isPositive, isReverse):
        self.key = key
        self.question = question
        self.questionType = type
        self.maxValue = maxValue
        self.minValue = minValue
        self.isVisibleInVisualizer = isVisibleInVisualizer
        self.buttons = buttons
        self.category = category
        self.displayName = displayName
        self.isPositive = isPositive
        self.isReverse = isReverse

    @classmethod
    def from_json(cls, data):
        try:
            # Validate the data structure
            key = data['key']
            question = data['question']
            questions_type = data['type']
            category = data['category']
            display_name = data['displayName']
            if 'isPositive' in data:
                is_positive = data['isPositive']
            else:
                is_positive = True

            if 'isReverse' in data:
                is_reverse = data['isReverse']
            else:
                is_reverse = False

            if 'buttons' in data:
                buttons = json.dumps(data['buttons'])
            else:
                buttons = None

            if questions_type == 'range':
                ranges = data['buttons']
                list_of_keys = list(ranges.keys())
                list_of_keys = [int(i) for i in list_of_keys]
                max_value = max(list_of_keys)
                min_value = min(list_of_keys)
            else:
                if questions_type == 'boolean':
                    max_value = 1
                    min_value = 0
                else:
                    max_value = None
                    min_value = None

            if 'isVisibleInVisualizer' in data:
                is_visible_in_visualizer = data['isVisibleInVisualizer']
            else:
                is_visible_in_visualizer = False

            return cls(key, question, questions_type, max_value, min_value, is_visible_in_visualizer, buttons, category, display_name, is_positive, is_reverse)
        except KeyError:
            raise ValueError('Invalid data structure')

    def __repr__(self):
        return f"Question(key={self.key}, question={self.question}, type={self.questionType}, maxValue={self.maxValue}, minValue={self.minValue}, isVisibleInVisualizer={self.isVisibleInVisualizer}, options={self.buttons}, category={self.category}, displayName={self.displayName}, isPositive={self.isPositive}, isReverse={self.isReverse})"


class Command:
    def __init__(self, name, description, schedule):
        self.name = name
        self.description = description
        self.schedule = schedule

    @classmethod
    def from_json(cls, data, name):
        try:
            # Validate the data structure
            description = data[name]['description']
            schedule = data[name]['schedule']
            return cls(name, description, schedule)
        except KeyError:
            raise ValueError('Invalid data structure')

    # make class printable

    def __repr__(self):
        return f"Command(name={self.name}, description={self.description}, schedule={self.schedule})"


questions = []
commands = []

r = requests.get(sys.argv[2])
data = r.json()
commands = [Command.from_json(data, name) for name in data]

for command in commands:
    # print(data[command.name]['questions'])
    # if question type not header
    commandQuestions = ([Question.from_json(
        question) for question in data[command.name]['questions'] if question['type'] != 'header'])

    for question in commandQuestions:
        questions.append(question)

# exit()
# Connect to the database
conn = psycopg2.connect(sys.argv[1])
cursor = conn.cursor()

#
# Create the table if it doesn't exist
table_name = 'questions'
# create_table_query = f"CREATE TABLE IF NOT EXISTS {table_name} (column1 key, column2 question, column3 type, column4 maxValue, column5 minValue, column6 isVisibleInVisualizer, column7 buttons);"
create_table_query = f"CREATE TABLE IF NOT EXISTS {table_name} (key VARCHAR(255), question VARCHAR(255), question_type VARCHAR(255), max_value int, min_value int, is_visible_in_visualizer BOOLEAN, buttons VARCHAR(255), category VARCHAR(255), display_name VARCHAR(255), is_positive BOOLEAN, is_reverse BOOLEAN);"
cursor.execute(create_table_query)
conn.commit()

# Clear the table if it exists
clear_table_query = f"DELETE FROM {table_name};"
cursor.execute(clear_table_query)
conn.commit()

for item in questions:
    # insert_query = f"INSERT INTO {table_name} VALUES ({item.key}, {item.question}, {item.type}, {item.maxValue}, {item.minValue}, {item.isVisibleInVisualizer}, {item.buttons});"
    # insert_query = f"INSERT INTO {table_name} VALUES ('{item.key}', '{item.question}', '{item.type}', '{item.maxValue}', '{item.minValue}', '{item.isVisibleInVisualizer}', '{item.buttons}');"
    insert_query = f"INSERT INTO {table_name} VALUES(%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s);"
    cursor.execute(insert_query, (item.key, item.question, item.questionType,
                   item.maxValue, item.minValue, item.isVisibleInVisualizer, item.buttons, item.category, item.displayName, item.isPositive, item.isReverse))


table_name = 'commands'
create_table_query = f"CREATE TABLE IF NOT EXISTS {table_name} (name VARCHAR(255), description VARCHAR(255), schedule VARCHAR(255));"
cursor.execute(create_table_query)
conn.commit()

# Clear the table if it exists
clear_table_query = f"DELETE FROM {table_name};"
cursor.execute(clear_table_query)
conn.commit()

for item in commands:
    insert_query = f"INSERT INTO {table_name} VALUES(%s, %s, %s);"
    cursor.execute(insert_query, (item.name, item.description, item.schedule))


# Add the data to the table


# Commit the changes and close the connection
conn.commit()
conn.close()
