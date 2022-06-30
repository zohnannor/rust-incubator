pub trait Command {}

pub trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}
