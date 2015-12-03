extern crate clap;

use clap::{App, Arg};

#[test]
fn opts_using_short() {
    let m = App::new("opts")
        .args(vec![
            Arg::from_usage("-f [flag] 'some flag'"),
            Arg::from_usage("-c [color] 'some other flag'")
            ])
        .get_matches_from(vec!["", "-f", "some", "-c", "other"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_long_space() {
    let m = App::new("opts")
        .args(vec![
            Arg::from_usage("--flag [flag] 'some flag'"),
            Arg::from_usage("--color [color] 'some other flag'")
            ])
        .get_matches_from(vec!["", "--flag", "some", "--color", "other"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_long_equals() {
    let m = App::new("opts")
        .args(vec![
            Arg::from_usage("--flag [flag] 'some flag'"),
            Arg::from_usage("--color [color] 'some other flag'")
            ])
        .get_matches_from(vec!["", "--flag=some", "--color=other"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_mixed() {
    let m = App::new("opts")
        .args(vec![
            Arg::from_usage("-f, --flag [flag] 'some flag'"),
            Arg::from_usage("-c, --color [color] 'some other flag'")
            ])
        .get_matches_from(vec!["", "-f", "some", "--color", "other"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");

    let m = App::new("opts")
        .args(vec![
            Arg::from_usage("-f, --flag [flag] 'some flag'"),
            Arg::from_usage("-c, --color [color] 'some other flag'")
            ])
        .get_matches_from(vec!["", "--flag=some", "-c", "other"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn create_option() {
    let _ = App::new("test")
                .arg(Arg::with_name("test")
                            .short("t")
                            .long("test")
                            .takes_value(true)
                            .help("testing testing"))
                .get_matches();
}

#[test]
fn create_option_usage() {
    // Short only
    let a = Arg::from_usage("[option] -o [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.short.unwrap(), 'o');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o [opt] 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert!(b.long.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert_eq!(&b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> -o <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.short.unwrap(), 'o');
    assert!(c.long.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o <opt> 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert!(d.long.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(d.num_vals.is_none());

    let a = Arg::from_usage("[option] -o [opt]... 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.short.unwrap(), 'o');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(a.num_vals.is_none());

    let a = Arg::from_usage("[option]... -o [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.short.unwrap(), 'o');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o [opt]... 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert!(b.long.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert_eq!(&b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> -o <opt>... 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.short.unwrap(), 'o');
    assert!(c.long.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let c = Arg::from_usage("<option>... -o <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.short.unwrap(), 'o');
    assert!(c.long.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o <opt>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert!(d.long.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(d.num_vals.is_none());

    // Long only

    let a = Arg::from_usage("[option] --opt [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("--opt [option] 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert_eq!(&b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("--opt <option> 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(d.num_vals.is_none());

    let a = Arg::from_usage("[option] --opt [opt]... 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(a.num_vals.is_none());

    let a = Arg::from_usage("[option]... --opt [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("--opt [option]... 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert_eq!(&b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> --opt <opt>... 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let c = Arg::from_usage("<option>... --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("--opt <option>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(d.num_vals.is_none());

    // Long only with '='

    let a = Arg::from_usage("[option] --opt=[opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("--opt=[option] 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert_eq!(&b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("--opt=<option> 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(d.num_vals.is_none());

    let a = Arg::from_usage("[option] --opt=[opt]... 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(a.num_vals.is_none());

    let a = Arg::from_usage("[option]... --opt=[opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("--opt=[option]... 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert_eq!(&b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> --opt=<opt>... 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let c = Arg::from_usage("<option>... --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("--opt=<option>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(d.num_vals.is_none());

    // Long and Short

    let a = Arg::from_usage("[option] -o --opt [option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o --opt [option] 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert_eq!(&b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> -o --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o --opt <option> 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(d.num_vals.is_none());

    let a = Arg::from_usage("[option]... -o --opt [option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o --opt [option]... 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert_eq!(&b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option>... -o --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o --opt <option>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(d.num_vals.is_none());

    // Long and Short with '='

    let a = Arg::from_usage("[option] -o --opt=[option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o --opt=[option] 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert_eq!(&b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option> -o --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o --opt=<option> 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(d.num_vals.is_none());

    let a = Arg::from_usage("[option]... -o --opt=[option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.multiple);
    assert!(a.takes_value);
    assert!(!a.required);
    assert_eq!(&a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("-o --opt=[option]... 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.multiple);
    assert!(b.takes_value);
    assert!(!b.required);
    assert_eq!(&b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("<option>... -o --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(c.takes_value);
    assert!(c.required);
    assert_eq!(&c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["opt"]);
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("-o --opt=<option>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["option"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_with_vals() {
    let d = Arg::from_usage("-o <file> <mode> 'some help info'");
    assert_eq!(d.name, "o");
    assert!(d.long.is_none());
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["file", "mode"]);
    assert_eq!(d.num_vals.unwrap(), 2);

    let d = Arg::from_usage("-o <file> <mode>... 'some help info'");
    assert_eq!(d.name, "o");
    assert!(d.long.is_none());
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["file", "mode"]);
    assert_eq!(d.num_vals.unwrap(), 2);

    let d = Arg::from_usage("--opt <file> <mode>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert!(d.short.is_none());
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["file", "mode"]);
    assert_eq!(d.num_vals.unwrap(), 2);

    let d = Arg::from_usage("[myopt] --opt <file> <mode> 'some help info'");
    assert_eq!(d.name, "myopt");
    assert!(d.short.is_none());
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(!d.required);
    assert_eq!(&d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), &["file", "mode"]);
    assert_eq!(d.num_vals.unwrap(), 2);

    let d = Arg::from_usage("--opt <file> <mode> 'some help info'");
    assert_eq!(d.name, "opt");
    assert!(d.short.is_none());
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.multiple);
    assert!(d.takes_value);
    assert!(d.required);
    assert_eq!(d.num_vals.unwrap(), 2);
}