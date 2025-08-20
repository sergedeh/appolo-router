#[test]
fn composite_field_with_subfields_from_another_service() {
    let planner = planner!(
        accounts: r#"
        type Query {
            me: User
        }

        type User @key(fields: "id") {
            id: ID!
            name: Name
            birthDate: String
        }

        type Name {
            first: String
        }
        "#,
        reviews: r#"
        type Query {
            topReviews: [Review!]!
        }

        type Review {
            body: String!
            author: User
        }

        type User @key(fields: "id") {
            id: ID!
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            topReviews {
              body
              author {
                name { first }
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "reviews") {
              {
                topReviews {
                  body
                  author {
                    __typename
                    id
                  }
                }
              }
            },
            Flatten(path: "topReviews.@.author") {
              Fetch(service: "accounts") {
                {
                  ... on User {
                    __typename
                    id
                  }
                } =>
                {
                  ... on User {
                    name {
                      first
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
fn composite_field_parent_selection_empty() {
    let planner = planner!(
        accounts: r#"
        type Query {
            me: User
        }

        type User @key(fields: "id") {
            id: ID!
            name: Name
            birthDate: String
        }

        type Name {
            first: String
        }
        "#,
        reviews: r#"
        type Query {
            topReviews: [Review!]!
        }

        type Review {
            author: User
        }

        type User @key(fields: "id") {
            id: ID!
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            topReviews {
              author {
                name { first }
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "reviews") {
              {
                topReviews {
                  author {
                    __typename
                    id
                  }
                }
              }
            },
            Flatten(path: "topReviews.@.author") {
              Fetch(service: "accounts") {
                {
                  ... on User {
                    __typename
                    id
                  }
                } =>
                {
                  ... on User {
                    name {
                      first
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
