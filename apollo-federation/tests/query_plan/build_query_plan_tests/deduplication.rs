#[test]
fn inline_fragment_duplication_deduplicated() {
    let planner = planner!(
        documents: r#"
        type Query {
            body: Body
        }

        union Body = Image | Text

        type Image {
            attributes: TextAttributes
        }

        type Text {
            attributes: TextAttributes
        }

        type TextAttributes {
            bold: Boolean
            text: String
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            body {
              ... on Body {
                ... on Text {
                  attributes {
                    bold
                    text
                  }
                }
              }
              ... on Text {
                attributes {
                  bold
                  text
                }
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Fetch(service: "documents") {
            {
              body {
                __typename
                ... on Text {
                  attributes {
                    bold
                    text
                  }
                }
              }
            }
          },
        }
        "###
    );
}

#[test]
fn named_fragment_spread_duplication_deduplicated() {
    let planner = planner!(
        documents: r#"
        type Query {
            body: Body
        }

        union Body = Image | Text

        type Image {
            attributes: TextAttributes
        }

        type Text {
            attributes: TextAttributes
        }

        type TextAttributes {
            bold: Boolean
            text: String
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          fragment TextFragment on Text {
            attributes {
              bold
              text
            }
          }

          query {
            body {
              ... on Body {
                ...TextFragment
              }
              ...TextFragment
            }
          }
        "#,
        @r###"
        QueryPlan {
          Fetch(service: "documents") {
            {
              body {
                __typename
                ... on Text {
                  attributes {
                    bold
                    text
                  }
                }
              }
            }
          },
        }
        "###
    );
}
