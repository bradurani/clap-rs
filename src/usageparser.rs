use std::str;

use vec_map::VecMap;

use args::Arg;

type ParseResult = Result<(), ()>;

#[derive(PartialEq, Debug)]
enum UsageToken {
    Name,
    ValName,
    Short,
    Long,
    Help,
    Multiple,
    Unknown
}

#[derive(Debug)]
pub struct UsageArg<'a> {
    usage: &'a [u8],
    pos: usize,
    start: usize,
    prev: UsageToken,
    explicit_name_set: bool
}

impl<'a, 'b, 'c, 'd> UsageArg<'a> {
    fn new(usage: &'a [u8]) -> Self {
        debugln!("exec=new; usage={:?}", usage);
        UsageArg {
            usage: usage,
            pos: 0,
            start: 0,
            prev: UsageToken::Unknown,
            explicit_name_set: false,
        }
    }

    pub fn from_usage(usage: &'a str) -> Self {
        debugln!("exec=from_usage; usage={:?}", usage);
        UsageArg::new(usage.as_bytes())
    }

    pub fn parse(mut self) -> Arg<'a, 'a, 'a, 'b, 'c , 'd> {
        debugln!("exec=parse;");
        let mut arg = Arg { empty_vals: true, ..Default::default() };
        loop {
            debugln!("iter; pos={};", self.pos);
            self.stop_at(token);
            if self.pos < self.usage.len() {
                let c = self.usage[self.pos];
                if c == b'-'       { self.short_or_long(&mut arg); } 
                else if c == b'.'  { self.multiple(&mut arg); } 
                else if c == b'\'' { self.help(&mut arg); } 
                else               { self.name(&mut arg); }
            } else { 
                break;
            }
        }
        if arg.name.len() == 0 { panic!("No name found for usage string: {}", str::from_utf8(self.usage).unwrap())}
        let n_vals = if let Some(ref v) = arg.val_names { v.len() } else { 0 };
        if n_vals > 1 {
            arg.num_vals = Some(n_vals as u8);
        }
        arg
    }

    fn name(&mut self, arg: &mut Arg<'a, 'a, 'a, 'b, 'c, 'd>) {
        debugln!("exec=name;");
        if self.usage[self.pos] == b'<' && !self.explicit_name_set { arg.required = true; }
        self.pos += 1;
        self.stop_at(name_end);
        let name = unsafe { str::from_utf8_unchecked(&self.usage[self.start..self.pos]) };
        if self.prev != UsageToken::Unknown {
            debugln!("setting val name...");
            if let Some(ref mut v) = arg.val_names {
                let len = v.len();
                v.insert(len, name);
            } else {
                let mut v = VecMap::new();
                v.insert(0, name);
                arg.val_names = Some(v);
                arg.takes_value = true;
            }
            self.prev = UsageToken::ValName;
        } else { 
            debugln!("setting name...");
            arg.name = name;
            if arg.long.is_none() && arg.short.is_none() {
                debugln!("explicit name set...");
                self.explicit_name_set = true;
                self.prev = UsageToken::Name;
            }
        }
    }

    fn stop_at<F>(&mut self, f: F) where F: Fn(u8) -> bool {
        debugln!("exec=stop_at;");
        self.start = self.pos;
        for b in &self.usage[self.start..] {
            if f(*b) { self.pos += 1; continue; }
            return;
        }
    }

    fn short_or_long(&mut self, arg: &mut Arg<'a, 'a, 'a, 'b, 'c, 'd>) {
        debugln!("exec=short_or_long;");
        self.pos += 1;
        if self.usage[self.pos] == b'-' {
            self.pos += 1;
            self.long(arg);
            return;
        }
        self.short(arg)
    }

    fn long(&mut self, arg: &mut Arg<'a, 'a, 'a, 'b, 'c, 'd>) {
        debugln!("exec=long;");
        self.stop_at(long_end);
        let name = unsafe { str::from_utf8_unchecked(&self.usage[self.start..self.pos]) };
        debugln!("var;name={}", name);
        if arg.name.len() == 0 || (self.prev == UsageToken::Short && arg.name.len() == 1) {
            arg.name = name;
        } 
        arg.long = Some(name);
        self.prev = UsageToken::Long;
    }

    fn short(&mut self, arg: &mut Arg<'a, 'a, 'a, 'b, 'c, 'd>) {
        debugln!("exec=short;");
        let name = unsafe { str::from_utf8_unchecked(&self.usage[self.pos..self.pos + 1]) };
        arg.short = Some(name.chars().nth(0).unwrap());
        if arg.name.len() == 0 {
            arg.name = name;
        }
        self.prev = UsageToken::Short;
    }

    fn multiple(&mut self, arg: &mut Arg) {
        debugln!("exec=multiple;");
        let mut dot_counter = 1;
        let start = self.pos;
        for b in &self.usage[start..] {
            if *b == b'.' {
                dot_counter += 1;
                self.pos += 1;
                if dot_counter == 3 {
                    arg.multiple = true;
                    self.prev = UsageToken::Multiple;
                    return
                }
            } else {
                return
            }
        }
    }

    fn help(&mut self, arg: &mut Arg<'a, 'a, 'a, 'b, 'c, 'd>) {
        debugln!("exec=help;");
        self.pos += 1;
        self.stop_at(help_end);
        arg.help = Some(unsafe { str::from_utf8_unchecked(&self.usage[self.start..self.pos]) });
        self.pos += 1;   // Move to next byte to keep from thinking ending ' is a start
        self.prev = UsageToken::Help;
    }
}

    #[inline]
    fn name_end(b: u8) -> bool {
        debugln!("exec=name_end; b={}", b);
        // 93(]), 62(>)
        b > b']' || b < b'>' || (b > b'>' && b < b']')
    }

    #[inline]
    fn token(b: u8) -> bool {
        debugln!("exec=token; b={}", b);
        // 39('), 45(-), 46(.), 60(<), 91([)
        b < 39 || b > 91 || (b > 46 && b < 91 && b != b'<') || (b > 39 && b < 45)
    }

    #[inline]
    fn long_end(b: u8) -> bool {
        debugln!("exec=long_end; b={}", b);
        // 39('), 46(.), 60(<), 61(=), 91([)
        (b < 39 && (b > 13 && b != b' ')) || b > 91 || (b > 61 && b < 91) || (b > 39 && b < 60 && b != 46)
    }

    #[inline]
    fn help_end(b: u8) -> bool {
        debugln!("exec=help_end; b={}", b);
        // 39(')
        b > 39 || b < 39
    }
