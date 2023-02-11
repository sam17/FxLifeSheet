import React, { useEffect, useState } from "react";
import CategoryData from "../models/category_data";
import QuestionData from "../models/question_data";
import styles from "../stylesheets.module.scss";
import { Divider, Row } from "antd";
import CalendarViz from "./CalendarViz";

interface QuestionsForCategory {
  category: string;
  questions: QuestionData[];
}

interface props {
  baseUrl: string;
}

function VizBuilder(props: props) {
  const { baseUrl } = props;
  const [categories, setCategories] = useState<CategoryData[]>([]);
  const [questionsForCategory, setQuestionsForCategory] = useState<
    QuestionsForCategory[]
  >([]);

  const getCategories = () => {
    return fetch(baseUrl + "categories").then((response) => response.json());
  };

  const getQuestionsForCategory = (category: string) => {
    return fetch(
      baseUrl + "questions?is_visible=true&category=" + category
    ).then((response) => response.json());
  };

  useEffect(() => {
    getCategories()
      .then((data) => {
        setCategories(data);
      })
      .catch((error) => {
        console.log(error);
      });
  }, []);

  useEffect(() => {
    console.log(categories);
    categories.forEach((element) => {
      getQuestionsForCategory(element.name)
        .then((data) => {
          setQuestionsForCategory((prev) => {
            // if element.category already exists, update it
            // else add it
            const index = prev.findIndex(
              (item) => item.category === element.name
            );
            if (index !== -1) {
              prev[index].questions = data;
              return prev;
            }
            return [...prev, { category: element.name, questions: data }];
          });
        })
        .catch((error) => {
          console.log(error);
        });
    });
  }, [categories]);

  useEffect(() => {
    console.log(questionsForCategory);
  }, [questionsForCategory]);

  return (
    <div>
      {questionsForCategory.map((item) => {
        return (
          <div>
            <Divider orientation="left" className={styles.divider}>
              {" "}
              {item.category}{" "}
            </Divider>
            <Row gutter={[16, 16]}>
                {item.questions.map((question) => {
                    return <CalendarViz
                    isPositive={true}
                    isReverse={false}
                    minRange={0}
                    maxRange={5}
                    name={question.key}
                    displayName={question.key}
                    url={baseUrl + "data/"}
                    />
                })}
            </Row>
            <br />
            <br />
          </div>
        );
      })}
    </div>
  );
}

export default VizBuilder;
