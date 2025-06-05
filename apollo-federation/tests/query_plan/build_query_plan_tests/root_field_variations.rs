#[test]
fn single_root_field_from_one_service() {
    let planner = planner!(
        Accounts: r#"
          type Query {
            me: User
          }

          type User @key(fields: "id") {
            id: ID!
            name: String
          }
        "#,
        Products: r#"
          type Query {
            product(id: ID!): Product
            topProducts: [Product]
          }

          type Product @key(fields: "id") {
            id: ID!
            name: String
            price: Int
          }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          { me { name } }
        "#,
        @r###"
        QueryPlan {
          Fetch(service: "Accounts") {
            {
              me {
                name
              }
            }
          },
        }
        "###
    );
}

#[test]
fn root_fields_from_different_services() {
    let planner = planner!(
        Accounts: r#"
          type Query {
            me: User
          }

          type User @key(fields: "id") {
            id: ID!
            name: String
          }
        "#,
        Products: r#"
          type Query {
            product(id: ID!): Product
            topProducts: [Product]
          }

          type Product @key(fields: "id") {
            id: ID!
            name: String
            price: Int
          }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          {
            me { name }
            topProducts { name }
          }
        "#,
        @r###"
        QueryPlan {
          Parallel {
            Fetch(service: "Accounts") {
              {
                me {
                  name
                }
              }
            },
            Fetch(service: "Products") {
              {
                topProducts {
                  name
                }
              }
            },
          },
        }
        "###
    );
}

#[test]
fn multiple_root_fields_from_same_service() {
    let planner = planner!(
        Accounts: r#"
          type Query {
            me: User
          }

          type User @key(fields: "id") {
            id: ID!
            name: String
          }
        "#,
        Products: r#"
          type Query {
            product(id: ID!): Product
            topProducts: [Product]
          }

          type Product @key(fields: "id") {
            id: ID!
            name: String
            price: Int
          }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          {
            product(id: "1") { name }
            topProducts { price }
          }
        "#,
        @r###"
        QueryPlan {
          Fetch(service: "Products") {
            {
              product(id: \"1\") {
                name
              }
              topProducts {
                price
              }
            }
          },
        }
        "###
    );
}

