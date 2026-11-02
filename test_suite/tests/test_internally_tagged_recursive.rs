use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Expr {
    Eq { field: String },
    And(Vec<Expr>),
    Not(Box<Expr>),
}

#[test]
fn test_internally_tagged_recursive_enum() {
    let eq = Expr::Eq { field: "x".into() };
    let json = serde_json::to_string(&eq).unwrap();
    assert!(json.contains(r#""type":"Eq"#));
    let roundtrip: Expr = serde_json::from_str(&json).unwrap();
    assert_eq!(eq, roundtrip);

    let eq_field = Expr::Eq {
        field: "test".into(),
    };
    let json = serde_json::to_string(&eq_field).unwrap();
    assert!(json.contains(r#""type":"Eq"#));
    let roundtrip: Expr = serde_json::from_str(&json).unwrap();
    assert_eq!(eq_field, roundtrip);

    let and = Expr::And(vec![Expr::Eq { field: "y".into() }]);
    let json = serde_json::to_string(&and).unwrap();
    let roundtrip: Expr = serde_json::from_str(&json).unwrap();
    assert_eq!(and, roundtrip);

    let not = Expr::Not(Box::new(Expr::Eq { field: "z".into() }));
    let json = serde_json::to_string(&not).unwrap();
    let roundtrip: Expr = serde_json::from_str(&json).unwrap();
    assert_eq!(not, roundtrip);

    let nested = Expr::And(vec![Expr::Not(Box::new(Expr::Eq { field: "a".into() }))]);
    let json = serde_json::to_string(&nested).unwrap();
    let roundtrip: Expr = serde_json::from_str(&json).unwrap();
    assert_eq!(nested, roundtrip);

    let json = r#"{"type":"And","And":[{"type":"Eq","field":"test"}]}"#;
    let expr: Expr = serde_json::from_str(json).unwrap();
    match expr {
        Expr::And(vec) => {
            assert_eq!(vec.len(), 1);
            match &vec[0] {
                Expr::Eq { field } => assert_eq!(field, "test"),
                Expr::And(_) | Expr::Not(_) => panic!("wrong variant"),
            }
        }
        Expr::Eq { .. } | Expr::Not(_) => panic!("wrong top-level variant"),
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
enum Container {
    Simple(String),
}

#[test]
fn test_internally_tagged_nonrecursive_newtype() {
    let simple = Container::Simple("value".into());
    let json = serde_json::to_string(&simple).unwrap();
    let roundtrip: Container = serde_json::from_str(&json).unwrap();
    assert_eq!(simple, roundtrip);
}
