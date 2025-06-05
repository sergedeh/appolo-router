#[test]
fn extension_field_from_another_service() {
    let planner = planner!(
        accounts: r#"
        type Query {
            me: User
        }

        type User @key(fields: "id") {
            id: ID!
            name: Name
        }

        type Name {
            first: String
        }
        "#,
        reviews: r#"
        type Review {
            body: String!
        }

        type User @key(fields: "id") {
            id: ID!
            reviews: [Review!]!
            numberOfReviews: Int!
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            me {
              name { first }
              reviews { body }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "accounts") {
              {
                me {
                  __typename
                  id
                  name {
                    first
                  }
                }
              }
            },
            Flatten(path: "me") {
              Fetch(service: "reviews") {
                {
                  ... on User {
                    __typename
                    id
                  }
                } =>
                {
                  ... on User {
                    reviews {
                      body
                    }
                  }
                }
              },
            },
          },
        }
        "###
    );
}

#[test]
fn extension_field_parent_selection_empty() {
    let planner = planner!(
        accounts: r#"
        type Query {
            me: User
        }

        type User @key(fields: "id") {
            id: ID!
            name: Name
        }

        type Name {
            first: String
        }
        "#,
        reviews: r#"
        type Review {
            body: String!
        }

        type User @key(fields: "id") {
            id: ID!
            reviews: [Review!]!
            numberOfReviews: Int!
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            me {
              reviews { body }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "accounts") {
              {
                me {
                  __typename
                  id
                }
              }
            },
            Flatten(path: "me") {
              Fetch(service: "reviews") {
                {
                  ... on User {
                    __typename
                    id
                  }
                } =>
                {
                  ... on User {
                    reviews {
                      body
                    }
                  }
                }
              },
            },
          },
        }
        "###
    );
}

#[test]
fn extension_field_only_adds_requirements_once() {
    let planner = planner!(
        accounts: r#"
        type Query {
            me: User
        }

        type User @key(fields: "id") {
            id: ID!
            name: Name
        }

        type Name {
            first: String
        }
        "#,
        reviews: r#"
        type Review {
            body: String!
        }

        type User @key(fields: "id") {
            id: ID!
            reviews: [Review!]!
            numberOfReviews: Int!
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            me {
              reviews { body }
              numberOfReviews
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "accounts") {
              {
                me {
                  __typename
                  id
                }
              }
            },
            Flatten(path: "me") {
              Fetch(service: "reviews") {
                {
                  ... on User {
                    __typename
                    id
                  }
                } =>
                {
                  ... on User {
                    reviews {
                      body
                    }
                    numberOfReviews
                  }
                }
              },
            },
          },
        }
        "###
    );
}
