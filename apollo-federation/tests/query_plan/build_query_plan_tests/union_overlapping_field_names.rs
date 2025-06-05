#[test]
fn union_type_with_overlapping_field_names() {
    let planner = planner!(
        Subgraph1: r#"
          type Query {
            search: Search
          }

          type A {
            id: ID!
            shared: Int
          }

          type B @key(fields: "id") {
            id: ID!
          }

          union Search = A | B
        "#,
        Subgraph2: r#"
          type B @key(fields: "id") {
            id: ID!
            shared: Int
          }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          {
            search {
              ... on A {
                shared
              }
              ... on B {
                shared
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "Subgraph1") {
              {
                search {
                  __typename
                  ... on A {
                    shared
                  }
                  ... on B {
                    __typename
                    id
                  }
                }
              }
            },
            Flatten(path: "search") {
              Fetch(service: "Subgraph2") {
                {
                  ... on B {
                    __typename
                    id
                  }
                } =>
                {
                  ... on B {
                    shared
                  }
                }
              },
            },
          },
        }
        "###
    );
}
