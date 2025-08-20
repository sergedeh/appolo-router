#[test]
fn relationship_subfields_from_same_service() {
    let planner = planner!(
        books: r#"
        type Book @key(fields: "id") {
          id: ID!
          title: String
        }
        "#,
        magazines: r#"
        type Magazine @key(fields: "id") {
          id: ID!
          title: String
        }
        "#,
        products: r#"
        type Query {
          products: [Product]
        }

        interface Product {
          id: ID!
          sku: String
          dimensions: ProductDimension
        }

        type ProductDimension @shareable {
          size: String
          weight: Float
        }

        type Book implements Product @key(fields: "id") {
          id: ID!
          sku: String
          dimensions: ProductDimension @shareable
        }

        type Magazine implements Product @key(fields: "id") {
          id: ID!
          sku: String
          dimensions: ProductDimension @shareable
        }
        "#,
        reviews: r#"
        type Book implements Product @key(fields: "id") {
          id: ID!
          reviews: [Review!]!
        }

        type Magazine implements Product @key(fields: "id") {
          id: ID!
          reviews: [Review!]!
        }

        interface Product {
          id: ID!
          reviews: [Review!]!
        }

        type Review {
          id: Int!
          body: String!
          product: Product
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          {
            products {
              reviews {
                body
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "products") {
              {
                products {
                  __typename
                  id
                }
              }
            },
            Flatten(path: "products.@") {
              Fetch(service: "reviews") {
                {
                  ... on Book {
                    __typename
                    id
                  }
                  ... on Magazine {
                    __typename
                    id
                  }
                } =>
                {
                  ... on Book {
                    reviews {
                      body
                    }
                  }
                  ... on Magazine {
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
fn relationship_subfields_with_provided_keys() {
    let planner = planner!(
        books: r#"
        type Book @key(fields: "id") {
          id: ID!
          title: String
        }
        "#,
        magazines: r#"
        type Magazine @key(fields: "id") {
          id: ID!
          title: String
        }
        "#,
        products: r#"
        type Query {
          products: [Product]
        }

        interface Product {
          id: ID!
          sku: String
          dimensions: ProductDimension
        }

        type ProductDimension @shareable {
          size: String
          weight: Float
        }

        type Book implements Product @key(fields: "id") {
          id: ID!
          sku: String
          dimensions: ProductDimension @shareable
        }

        type Magazine implements Product @key(fields: "id") {
          id: ID!
          sku: String
          dimensions: ProductDimension @shareable
        }
        "#,
        reviews: r#"
        type Book implements Product @key(fields: "id") {
          id: ID!
          reviews: [Review!]!
        }

        type Magazine implements Product @key(fields: "id") {
          id: ID!
          reviews: [Review!]!
        }

        interface Product {
          id: ID!
          reviews: [Review!]!
        }

        type Review {
          id: Int!
          body: String!
          product: Product
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          {
            products {
              reviews {
                product {
                  sku
                }
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "products") {
              {
                products {
                  __typename
                  id
                }
              }
            },
            Flatten(path: "products.@") {
              Fetch(service: "reviews") {
                {
                  ... on Book {
                    __typename
                    id
                  }
                  ... on Magazine {
                    __typename
                    id
                  }
                } =>
                {
                  ... on Book {
                    reviews {
                      product {
                        __typename
                        id
                      }
                    }
                  }
                  ... on Magazine {
                    reviews {
                      product {
                        __typename
                        id
                      }
                    }
                  }
                }
              },
            },
            Flatten(path: "products.@.reviews.@.product") {
              Fetch(service: "products") {
                {
                  ... on Book {
                    ... on Book {
                      __typename
                      id
                    }
                  }
                  ... on Magazine {
                    __typename
                    id
                  }
                } =>
                {
                  ... on Book {
                    sku
                  }
                  ... on Magazine {
                    sku
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
fn relationship_field_with_extension_subfields() {
    let planner = planner!(
        accounts: r#"
        type User @key(fields: "id") {
            id: ID!
            birthDate: String
        }
        "#,
        reviews: r#"
        type Query {
            topReviews: [Review]
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
                birthDate
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
                    birthDate
                  }
                }
              },
            },
          },
        }
        "###
    );
}
