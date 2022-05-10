/// This macro is used to parse a **sequence** of operands. Returns a `Vec<Node>`
#[macro_export]
macro_rules! answer_parse_seq {
    // Base case, no tokens means we've reached the end of the sequence, return an empty vec
    () => ({
        vec![]
    });
    // Match `, <sequence of tokens>`. Just drop the comma and recursively parse whatever follows.
    //
    // This approach has the downside that it doesn't explicitly reject duplicated commas (it just
    // takes two recursion steps to drop both of them) but for this simple example I think it's
    // fine
    ( , $( $tail:tt )* ) => ({
        $crate::answer_parse_seq!( $($tail)* )
    });
    // Match `v: <sequence of tokens>`
    //
    // Parse the tail as a sequence of nodes, then apply the verify wrapper to the first node in
    // the vec
    ( v: $( $tail:tt )* ) => ({
        let mut v = $crate::answer_parse_seq!( $( $tail )* );
        v[0] = Node::Verify(Box::new(v[0].clone()));
        v
    });
    // Match `<something1> <something2> <tail>`
    //
    // Parse using the single-node macro `<something1> ( <something2> )` where usually `<something1>`
    // is the name of an operand (like `pk`) and `<something2>` its set of arguments. Then parse
    // the tail as a sequence and preprend it with the single node parsed
    ( $op:tt $arg:tt $( $tail:tt )* ) => ({
        let mut v = vec![$crate::answer_parse!( $op ( $arg ) )];
        v.extend($crate::answer_parse_seq!( $( $tail )* ));
        v
    });
}

/// This macro is used to parse operands. Every branch returns a single `Node` instance
#[macro_export]
macro_rules! answer_parse {
    // Match `pk ( <expression> )`
    //
    // Calls `.into()` on the content of the parenthesis to turn that type into a `String`
    ( pk ( $key:expr ) ) => ({
        Node::Pk($key.into())
    });
    // Match `after ( <expression > )`
    ( after ( $timelock:expr ) ) => ({
        Node::After($timelock)
    });
    // Match `v : <sequence of tokens>`
    //
    // Recurisvely uses this macro to parse the sequence of tokens that follows the colon,
    // and then puts that result into a `Verify` node
    ( v: $( $tt:tt )+ ) => ({
        Node::Verify(Box::new($crate::answer_parse!( $($tt)* )))
    });
    // Match `thresh ( <expr>, <sequence of tokens> )`
    //
    // The first expression is the threshold, then uses a different macro (`answer_parse_seq`) to
    // parse a **sequence** of nodes
    ( thresh ( $t:expr, $( $body:tt )* ) ) => ({
        Node::Thresh($t, $crate::answer_parse_seq!( $($body)* ))
    });
}
