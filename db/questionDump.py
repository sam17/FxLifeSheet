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
            questionsType = data['type']
            category = data['category']
            displayName = data['displayName']
            if 'isPositive' in data:
                isPositive = data['isPositive']
            else:
                isPositive = True

            if 'isReverse' in data:
                isReverse = data['isReverse']
            else:
                isReverse = False

            if 'buttons' in data:
                buttons = json.dumps(data['buttons'])
            else:
                buttons = None

            if questionsType == 'range':
                ranges = data['buttons']
                listOfKeys = list(ranges.keys())
                listOfKeys = [int(i) for i in listOfKeys]
                maxValue = max(listOfKeys)
                minValue = min(listOfKeys)
            else:
                if questionsType == 'boolean':
                    maxValue = 1
                    minValue = 0
                else:
                    maxValue = None
                    minValue = None

            if 'isVisibleInVisualizer' in data:
                isVisibleInVisualizer = data['isVisibleInVisualizer']
            else:
                isVisibleInVisualizer = False

            return cls(key, question, questionsType, maxValue, minValue, isVisibleInVisualizer, buttons, category, displayName, isPositive, isReverse)
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

r = requests.get(
    'https://raw.githubusercontent.com/thebayesianconspiracy/FxLifeSheet/master/telegram_bot/lifesheet.json')
    # 'https://raw.githubusercontent.com/thebayesianconspiracy/FxLifeSheet/seperateJSON/telegram_bot/lifesheet.json')
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
