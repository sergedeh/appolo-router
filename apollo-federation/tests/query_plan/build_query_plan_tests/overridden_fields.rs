#[test]
fn overridden_field_and_type() {
    let planner = planner!(
        books: r#"
        type Query {
            library(id: ID!): Library
        }

        type Library @key(fields: "id") {
            id: ID!
            name: String
        }
        "#,
        accounts: r#"
        type Library @key(fields: "id") {
            id: ID!
            description: String
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            library(id: "3") {
              name
              description
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "books") {
              {
                library(id: 3) {
                  __typename
                  id
                  name
                }
              }
            },
            Flatten(path: "library") {
              Fetch(service: "accounts") {
                {
                  ... on Library {
                    __typename
                    id
                  }
                } =>
                {
                  ... on Library {
                    description
                  }
                }
              },
            },
          },
        }
        "###
    );
}
