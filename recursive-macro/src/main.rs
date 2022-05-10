use std::str::FromStr;

mod parser;
#[macro_use]
mod answer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    /// `pk(KEY)`
    Pk(String),
    /// `after(VALUE)`
    After(u32),
    /// `v:`
    Verify(Box<Node>),
    /// `thresh(THRESH, NODE LIST...)`
    Thresh(u32, Vec<Node>),
}

macro_rules! parse {
    () => {};
}

#[test]
fn test_recursive_macro_pk() -> Result<(), Box<dyn std::error::Error>> {
    let string_parser = Node::from_str("pk(key")?;
    let macro_parser = parse!(pk("key"));

    assert_eq!(string_parser, macro_parser);
    Ok(())
}

#[test]
fn test_recursive_macro_wrapper() -> Result<(), Box<dyn std::error::Error>> {
    let string_parser = Node::from_str("v:pk(key")?;
    let macro_parser = parse!(v:pk("key"));

    assert_eq!(string_parser, macro_parser);
    Ok(())
}

#[test]
fn test_recursive_macro_multiple_wrappers() -> Result<(), Box<dyn std::error::Error>> {
    let string_parser = Node::from_str("v:v:v:pk(key")?;
    let macro_parser = parse!(v:v:v:pk("key"));

    assert_eq!(string_parser, macro_parser);
    Ok(())
}

#[test]
fn test_recursive_macro_thresh_without_wrappers() -> Result<(), Box<dyn std::error::Error>> {
    let string_parser = Node::from_str("thresh(2,pk(alice),after(10))")?;
    let macro_parser = parse!(thresh(2, pk("alice"), after(10)));

    assert_eq!(string_parser, macro_parser);
    Ok(())
}

#[test]
fn test_recursive_macro_thresh_wrapper() -> Result<(), Box<dyn std::error::Error>> {
    let string_parser = Node::from_str("thresh(2,v:pk(alice),after(10))")?;
    let macro_parser = parse!(thresh(2,v:pk("alice"),after(10)));

    assert_eq!(string_parser, macro_parser);
    Ok(())
}

#[test]
fn test_recursive_macro_thresh_multiple_wrappers() -> Result<(), Box<dyn std::error::Error>> {
    let string_parser = Node::from_str("thresh(2,v:v:pk(alice),v:after(10))")?;
    let macro_parser = parse!(thresh(2,v:v:pk("alice"),v:after(10)));

    assert_eq!(string_parser, macro_parser);
    Ok(())
}
