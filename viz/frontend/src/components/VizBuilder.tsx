import React, { useEffect, useState } from "react";
import CategoryData from "../models/category_data";
import QuestionData from "../models/question_data";
import styles from "../stylesheets.module.scss";
import { Divider, Row } from "antd";
import CalendarViz from "./CalendarViz";
import LineChartViz from "./LineChartViz";
import Tooltip from "./Tooltip";

interface QuestionsForCategory {
  category: string;
  questions: QuestionData[];
}

interface props {
  baseUrl: string;
}

interface tooltipData {
  visible: boolean;
  content: string;
}

function VizBuilder(props: props) {
  const { baseUrl } = props;
  const [categories, setCategories] = useState<CategoryData[]>([]);
  const [questionsForCategory, setQuestionsForCategory] = useState<
    QuestionsForCategory[]
  >([]);
  const [tooltipData, setTooltipData] = useState<tooltipData>({
    visible: false,
    content: "",
  });

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
    categories.forEach((element) => {
      getQuestionsForCategory(element.name)
        .then((data) => {
          setQuestionsForCategory((prev) => {
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
  }, [questionsForCategory]);

  return (
    <div className="VizBuilder">
      <Tooltip tooltipData={tooltipData} />
      {questionsForCategory.map((item) => {
        return (
          <div>
            <Divider orientation="left" className={styles.divider}>
              {" "}
              {item.category}{" "}
            </Divider>
            <Row gutter={[16, 16]}>
              {item.questions.map((question) => {
                if (question.graph_type === "line") {
                  return (
                    <LineChartViz
                      key = {question.key}
                      isPositive={question.is_positive}
                      minRange={question.min_value}
                      maxRange={question.max_value}
                      name={question.key}
                      displayName={question.display_name}
                      url={baseUrl + "data/"}
                      setTooltipData={setTooltipData}
                    />
                  );
                } else {
                  return (
                    <CalendarViz
                      key = {question.key}
                      isPositive={question.is_positive}
                      isReverse={question.is_reverse}
                      minRange={question.min_value}
                      maxRange={question.max_value}
                      name={question.key}
                      displayName={question.display_name}
                      url={baseUrl + "data/"}
                      cadence={question.cadence}
                    />
                  );
                }
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
