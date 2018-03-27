struct Context<'a>(&'a str);

/// Syntax for telling that lifetime of
/// Parser struct is longer than the lifetime
/// of context.
struct Parser<'c, 's : 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    ///
    fn parse(&self) -> Result<(), &'s str>
    {
        Err(&self.context.0[1..])
    }
}
/// Below definition of function parse_context will
/// fail to compile because, as per the definition of
/// the Parser structure, the Context has a lifetime same
/// as that of the Parser. But, here we are returning the part
/// of data associated with the context object (the string).
/*
fn parse_context(ctx : Context) -> Result<(), &str>
{
    Parser{context : &ctx}.parse()
}
*/

fn main() {
}
