struct Context<'s>(&'s str);

struct Parser<'c,'s:'c>{ // 's:'c means that 's will live at least as long as  'c
    context: &'c Context<'s>,
}

impl<'c,'s> Parser<'c,'s> {
    fn parse(&self) -> Result<(), &'s str>{
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str>{
    Parser { context: &context }.parse()
}
