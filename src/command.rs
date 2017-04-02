use combine::{choice, eof, many1, optional, Parser, parser, try, value};
use combine::char::{digit, hex_digit, space, spaces, string};
use combine::primitives::{ParseResult, Stream};

use std::str::{self, FromStr};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Command {
    ShowRegs,
    Step(usize),
    Continue,
    Goto(u16),
    ShowMem(Option<u16>),
    Disassemble(usize),
    Breakpoint,
    AddBreakpoint(u16),
    RemoveBreakpoint(u16),
    Watchpoint,
    AddWatchpoint(u16),
    RemoveWatchpoint(u16),
    Exit,
    Repeat,
}

impl FromStr for Command {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parser(command).parse(s) {
            Ok((c, _)) => Ok(c),
            err => Err(format!("Unable to parse command: {:?}", err).into()),
        }
    }
}

fn command<I: Stream<Item = char>>(input: I) -> ParseResult<Command, I> {
    let show_regs =
        choice([try(string("showregs")), try(string("r"))]).map(|_| Command::ShowRegs).boxed();

    let step =
        (choice([try(string("step")), try(string("s")), try(string("next")), try(string("n"))]),
         optional((spaces(), usize_()).map(|x| x.1)))
                .map(|(_, count)| Command::Step(count.unwrap_or(1)))
                .boxed();

    let continue_ =
        choice([try(string("continue")), try(string("c"))]).map(|_| Command::Continue).boxed();

    let goto = (choice([try(string("goto")), try(string("g"))]), spaces(), u16_hex())
        .map(|(_, _, addr)| Command::Goto(addr))
        .boxed();

    let show_mem = (choice([try(string("showmem")), try(string("m"))]),
                    optional((spaces(), u16_hex()).map(|x| x.1)))
            .map(|(_, addr)| Command::ShowMem(addr))
            .boxed();

    let disassemble = (choice([try(string("disassemble")), try(string("d"))]),
                       optional((spaces(), usize_()).map(|x| x.1)))
            .map(|(_, count)| Command::Disassemble(count.unwrap_or(4)))
            .boxed();

    let breakpoint =
        choice([try(string("breakpoint")), try(string("b"))]).map(|_| Command::Breakpoint).boxed();

    let add_breakpoint =
        (choice([try(string("addbreakpoint")), try(string("ab"))]), space(), u16_hex())
            .map(|(_, _, addr)| Command::AddBreakpoint(addr))
            .boxed();

    let remove_breakpoint =
        (choice([try(string("removebreakpoint")), try(string("rb"))]), space(), u16_hex())
            .map(|(_, _, addr)| Command::RemoveBreakpoint(addr))
            .boxed();

    let watchpoint =
        choice([try(string("watchpoint")), try(string("w"))]).map(|_| Command::Watchpoint).boxed();

    let add_watchpoint =
        (choice([try(string("addwatchpoint")), try(string("aw"))]), space(), u16_hex())
            .map(|(_, _, addr)| Command::AddWatchpoint(addr))
            .boxed();

    let remove_watchpoint =
        (choice([try(string("removewatchpoint")), try(string("rw"))]), space(), u16_hex())
            .map(|(_, _, addr)| Command::RemoveWatchpoint(addr))
            .boxed();

    let exit = choice([try(string("exit")),
                       try(string("quit")),
                       try(string("e")),
                       try(string("x")),
                       try(string("q"))])
            .map(|_| Command::Exit)
            .boxed();

    let repeat = value(Command::Repeat).boxed();

    choice(vec![show_regs,
                step,
                continue_,
                goto,
                show_mem,
                disassemble,
                breakpoint,
                add_breakpoint,
                remove_breakpoint,
                watchpoint,
                add_watchpoint,
                remove_watchpoint,
                exit,
                repeat]
                   .into_iter()
                   .map(|parser| (parser, eof()).map(|x| x.0))
                   .map(try)
                   .collect::<Vec<_>>())
            .parse_stream(input)
}

fn usize_<'a, I: Stream<Item = char> + 'a>() -> Box<Parser<Input = I, Output = usize> + 'a> {
    many1(digit()).and_then(|s: String| s.parse::<usize>()).boxed()
}

fn u16_hex<'a, I: Stream<Item = char> + 'a>() -> Box<Parser<Input = I, Output = u16> + 'a> {
    let hex_prefix = choice([try(string("0x")), try(string("$"))]);
    (optional(hex_prefix), many1(hex_digit()))
        .map(|x| x.1)
        .and_then(|s: String| u16::from_str_radix(&s, 16))
        .boxed()
}
