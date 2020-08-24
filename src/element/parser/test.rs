use {
    nom::{
        error::ErrorKind,
        multi::many0,
        IResult,
    },
    std::collections::HashMap,
    super::{
        decode::{
            line2,
            triangle3,
            self,
        },
        Element,
        Elements,
        ElementTag,
        Entity,
    },
};

#[test]
fn elements() {
    let content = "\
$Elements
1 2 1 2
2 1 2 2
1 1 2 3
2 3 4 5
$EndElements
";
    let mut es : HashMap<ElementTag, Element> = HashMap::new();
    es.insert(1, Element::Triangle3(1, 2, 3));
    es.insert(2, Element::Triangle3(3, 4, 5));

    let entities = Entity::new(2, 1, 2, es);
    let expected = Elements::new(1, 2, vec![entities]);
    println!("{:?}", expected);
    match decode::elements::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(e) => {
            eprintln!("{:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn entity_line2() {
    let content = "\
2 1 1 2
1 1 2
2 3 4
";
    let mut elements : HashMap<ElementTag, Element> = HashMap::new();
    elements.insert(1, Element::Line2(1, 2));
    elements.insert(2, Element::Line2(3, 4));

    let expected = Entity::new(2, 1, 1, elements);
    match decode::entity::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn entity_triangle3() {
    let content = "\
2 1 2 2
1 1 2 3
2 4 5 6
";
    let mut elements : HashMap<ElementTag, Element> = HashMap::new();
    elements.insert(1, Element::Triangle3(1, 2, 3));
    elements.insert(2, Element::Triangle3(4, 5, 6));

    let expected = Entity::new(2, 1, 2, elements);
    match decode::entity::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn elements_line2() {
    let content = "\
1 1 2
2 3 4
3 5 6
";
    let expected = vec![
        (1, Element::Line2(1, 2)),
        (2, Element::Line2(3, 4)),
        (3, Element::Line2(5, 6)),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<(ElementTag, Element)>> {
        many0(line2)(s)
    }
    match parser(content) {
        Ok((i_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for ((et, ee), (at, ae)) in expected.iter().zip(actual) {
                assert_eq!(*et, at);
                assert_eq!(*ee, ae);
            }
        }
        Err(_) => assert!(false),
    }
}


#[test]
fn elements_triangle3() {
    let content = "\
1 1 2 3
2 4 5 6
3 7 8 9
";
    let expected = vec![
        (1, Element::Triangle3(1, 2, 3)),
        (2, Element::Triangle3(4, 5, 6)),
        (3, Element::Triangle3(7, 8, 9)),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<(ElementTag, Element)>> {
        many0(triangle3)(s)
    }
    match parser(content) {
        Ok((i_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for ((et, ee), (at, ae)) in expected.iter().zip(actual) {
                assert_eq!(*et, at);
                assert_eq!(*ee, ae);
            }
        }
        Err(_) => assert!(false),
    }
}
