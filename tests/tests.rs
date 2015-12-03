#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use std::vec::Vec;

arg_enum!{
    #[derive(Debug)]
    enum Val1 {
        ValOne,
        ValTwo
    }
}
arg_enum!{
    #[derive(Debug)]
    pub enum Val2 {
        ValOne,
        ValTwo
    }
}
arg_enum!{
    enum Val3 {
        ValOne,
        ValTwo
    }
}
arg_enum!{
    pub enum Val4 {
        ValOne,
        ValTwo
    }
}

#[test]
#[cfg_attr(feature = "lints", allow(single_match))]
fn test_enums() {
    let v1_lower = "valone";
    let v1_camel = "ValOne";

    let v1_lp = v1_lower.parse::<Val1>().unwrap();
    let v1_cp = v1_camel.parse::<Val1>().unwrap();
    match v1_lp {
        Val1::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val1::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val2>().unwrap();
    let v1_cp = v1_camel.parse::<Val2>().unwrap();
    match v1_lp {
        Val2::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val2::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val3>().unwrap();
    let v1_cp = v1_camel.parse::<Val3>().unwrap();
    match v1_lp {
        Val3::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val3::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val4>().unwrap();
    let v1_cp = v1_camel.parse::<Val4>().unwrap();
    match v1_lp {
        Val4::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val4::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
}

#[test]
fn create_app() {
    let _ =
        App::new("test").version("1.0").author("kevin").about("does awesome things").get_matches();
}

#[test]
fn add_multiple_arg() {
    let _ = App::new("test")
                .args( vec![
                    Arg::with_name("test").short("s"),
                    Arg::with_name("test2").short("l")])
                .get_matches();
}

#[test]
fn create_args_tabs_usage() {
    let a = Arg::from_usage("[pos]\t'some help info'");
    assert_eq!(a.name, "pos");
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("<pos>\t'some help info'");
    assert_eq!(b.name, "pos");
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("[pos]...\t'some help info'");
    assert_eq!(c.name, "pos");
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(!c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("<pos>...\t'some help info'");
    assert_eq!(d.name, "pos");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());
}

#[test]
fn create_subcommand() {
    let _ = App::new("test")
                .subcommand(SubCommand::with_name("some")
                                        .arg(Arg::with_name("test")
                                            .short("t")
                                            .long("test")
                                            .takes_value(true)
                                            .help("testing testing")))
                .arg(Arg::with_name("other").long("other"))
                .get_matches();
}

#[test]
fn create_multiple_subcommands() {
    let _ = App::new("test")
                .subcommands(vec![ SubCommand::with_name("some")
                                        .arg(Arg::with_name("test")
                                            .short("t")
                                            .long("test")
                                            .takes_value(true)
                                            .help("testing testing")),
                                    SubCommand::with_name("add")
                                        .arg(Arg::with_name("roster").short("r"))])
                .arg(Arg::with_name("other").long("other"))
                .get_matches();
}
