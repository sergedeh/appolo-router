// Abstract type related tests
#[test]
fn adds_typename_when_fetching_interface_fields() {
    let planner = planner!(
        product: r#"
        type Query {
            topProducts: [Product]
        }

        interface Product {
            price: Int
        }

        type Book implements Product @key(fields: "id") {
            id: ID!
            price: Int
        }

        type Furniture implements Product @key(fields: "id") {
            id: ID!
            price: Int
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            topProducts {
              price
            }
          }
        "#,
        @r###"
        QueryPlan {
          Fetch(service: "product") {
            {
              topProducts {
                __typename
                price
              }
            }
          },
        }
        "###
    );
}

#[test]
fn handles_fragment_spreads_multiple_times_correctly() {
    let planner = planner!(
        product: r#"
        type Query {
            topProducts: [Product]
        }

        interface Product {
            price: Int
            details: ProductDetails
        }

        type ProductDetails {
            country: String
        }

        type Book implements Product @key(fields: "id") {
            id: ID!
            price: Int
            details: ProductDetails
        }

        type Furniture implements Product @key(fields: "id") {
            id: ID!
            price: Int
            details: ProductDetails
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          fragment PriceAndCountry on Product {
            price
            details {
              country
            }
          }

          query {
            topProducts {
              __typename
              ... on Book {
                ...PriceAndCountry
              }
              ... on Furniture {
                ...PriceAndCountry
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Fetch(service: "product") {
            {
              topProducts {
                __typename
                ... on Book {
                  ...PriceAndCountry
                }
                ... on Furniture {
                  ...PriceAndCountry
                }
              }
            }

            fragment PriceAndCountry on Product {
              price
              details {
                __typename
                country
              }
            }
          },
        }
        "###
    );
}

#[test]
fn handles_repeated_inline_fragments_correctly() {
    let planner = planner!(
        product: r#"
        type Query {
            topProducts: [Product]
        }

        interface Product {
            price: Int
        }

        type Book implements Product @key(fields: "id") {
            id: ID!
            price: Int
        }

        type Furniture implements Product @key(fields: "id") {
            id: ID!
            price: Int
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            topProducts {
              __typename
              ... on Book {
                ... on Product {
                  price
                }
              }
              ... on Furniture {
                ... on Product {
                  price
                }
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Fetch(service: "product") {
            {
              topProducts {
                __typename
                ... on Book {
                  price
                }
                ... on Furniture {
                  price
                }
              }
            }
          },
        }
        "###
    );
}

#[test]
fn eliminates_unnecessary_type_conditions() {
    let planner = planner!(
        documents: r#"
        type Query {
            body: Body
        }

        union Body = Image | Text

        interface NamedObject {
            name: String
        }

        type Image implements NamedObject {
            name: String
        }

        type Text implements NamedObject {
            name: String
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            body {
              ... on Image {
                ... on NamedObject {
                  name
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
                ... on Image {
                  name
                }
              }
            }
          },
        }
        "###
    );
}

#[test]
fn keeps_directives_on_redundant_inline_fragments() {
    let planner = planner!(
        documents: r#"
        type Query {
            body: Body
        }

        union Body = Image | Text

        interface NamedObject {
            name: String
        }

        type Image implements NamedObject {
            name: String
        }

        type Text implements NamedObject {
            name: String
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query myQuery($b: Boolean!) {
            body {
              ... on Image {
                ... on NamedObject @include(if: $b) {
                  name
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
                ... on Image {
                  ... on NamedObject @include(if: $b) {
                    __typename
                    name
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
fn extension_field_on_interface_type() {
    let planner = planner!(
        product: r#"
        type Query {
            topProducts: [Product]
        }

        interface Product {
            price: Int
        }

        type Book implements Product @key(fields: "isbn") {
            isbn: String!
            price: Int
        }

        type Furniture implements Product @key(fields: "upc") {
            upc: String!
            price: Int
        }
        "#,
        reviews: r#"
        type Review {
            body: String
        }

        interface Product {
            reviews: [Review]
        }

        type Book @key(fields: "isbn") {
            isbn: String! @external
            reviews: [Review]
        }

        type Furniture @key(fields: "upc") {
            upc: String! @external
            reviews: [Review]
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            topProducts {
              price
              reviews {
                body
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "product") {
              {
                topProducts {
                  __typename
                  price
                  ... on Book {
                    __typename
                    isbn
                  }
                  ... on Furniture {
                    __typename
                    upc
                  }
                }
              }
            },
            Flatten(path: "topProducts.@") {
              Fetch(service: "reviews") {
                {
                  ... on Book {
                    __typename
                    isbn
                  }
                  ... on Furniture {
                    __typename
                    upc
                  }
                } =>
                {
                  ... on Book {
                    reviews {
                      body
                    }
                  }
                  ... on Furniture {
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
fn interface_fragments_expand_correctly() {
    let planner = planner!(
        books: r#"
        type Query {
            books: [Book]
        }

        type Book @key(fields: "isbn") {
            isbn: String!
            title: String
            year: Int
        }
        "#,
        product: r#"
        interface Product {
            name: String
        }

        type Book implements Product @key(fields: "isbn") {
            isbn: String! @external
            title: String @external
            year: Int @external
            name: String @requires(fields: "title year")
        }

        type Furniture implements Product @key(fields: "upc") {
            upc: String!
            name: String
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            books {
              ... on Product {
                name
                ... on Furniture {
                  upc
                }
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Sequence {
            Fetch(service: "books") {
              {
                books {
                  __typename
                  isbn
                  title
                  year
                }
              }
            },
            Flatten(path: "books.@") {
              Fetch(service: "product") {
                {
                  ... on Book {
                    __typename
                    isbn
                    title
                    year
                  }
                } =>
                {
                  ... on Book {
                    name
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
fn interface_nested_inside_interface() {
    let planner = planner!(
        product: r#"
        type Query {
            product(upc: String!): Product
        }

        interface Product {
            details: ProductDetails
        }

        interface ProductDetails {
            country: String
        }

        type Book implements Product @key(fields: "isbn") {
            isbn: String!
            details: BookDetails
        }

        type BookDetails implements ProductDetails {
            country: String
        }

        type Furniture implements Product @key(fields: "upc") {
            upc: String!
            details: FurnitureDetails
        }

        type FurnitureDetails implements ProductDetails {
            country: String
        }
        "#,
    );
    assert_plan!(
        &planner,
        r#"
          query {
            product(upc: "") {
              details {
                country
              }
            }
          }
        "#,
        @r###"
        QueryPlan {
          Fetch(service: "product") {
            {
              product(upc: "") {
                __typename
                details {
                  __typename
                  country
                }
              }
            }
          },
        }
        "###
    );
}

#[test]
fn nested_unions_with_inline_fragments() {
    let planner = planner!(
        documents: r#"
        type Query {
            body: Body
        }

        union Body = Image | Text

        type Image {
            attributes: ImageAttributes
        }

        type Text {
            attributes: TextAttributes
        }

        type ImageAttributes {
            url: String
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
              ... on Image {
                ... on Body {
                  ... on Image {
                    attributes {
                      url
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
              ... on Text {
                attributes {
                  bold
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
                ... on Image {
                  attributes {
                    url
                  }
                }
                ... on Text {
                  attributes {
                    bold
                  }
                }
              }
            }
          },
        }
        "###
    );
}
