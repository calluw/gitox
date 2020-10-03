use crate::base;
use crate::data;
use crate::data::ObjectType;
use std::fs;
use std::io::{Result};
use std::process::exit;

pub fn handle(matches: clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("init", Some(submatches)) => init(submatches),
        ("hash-file", Some(submatches)) => hash_file(submatches),
        ("cat-file", Some(submatches)) => cat_file(submatches),
        ("write-tree", Some(submatches)) => write_tree(submatches),
        ("read-tree", Some(submatches)) => read_tree(submatches),
        ("commit", Some(submatches)) => commit(submatches),
        ("log", Some(submatches)) => log(submatches),
        ("checkout", Some(submatches)) => checkout(submatches),
        ("tag", Some(submatches)) => tag(submatches),
        _ => {
            eprintln!("{}", matches.usage());
            exit(1);
        }
    }
}

fn init(_submatches: &clap::ArgMatches<'_>) -> Result<()> {
    data::init()
}

fn hash_file(submatches: &clap::ArgMatches<'_>) -> Result<()> {
    let oid = data::hash_object(
        &fs::read(submatches.value_of("FILE").unwrap())?,
        ObjectType::Blob,
    )?;
    println!("{}", oid);
    Ok(())
}

fn cat_file(submatches: &clap::ArgMatches<'_>) -> Result<()> {
    let oid = &base::get_oid(submatches.value_of("OID").unwrap())?;
    let object = data::get_object(&oid, None)?;
    print!("{}", String::from_utf8_lossy(&object.contents));
    Ok(())
}

fn write_tree(_submatches: &clap::ArgMatches<'_>) -> Result<()> {
    println!("{}", base::write_tree(".")?);
    Ok(())
}

fn read_tree(submatches: &clap::ArgMatches<'_>) -> Result<()> {
    base::read_tree(&base::get_oid(submatches.value_of("OID").unwrap())?)
}

fn commit(submatches: &clap::ArgMatches<'_>) -> Result<()> {
    let message = submatches.value_of("message").unwrap();
    println!("{}", base::commit(message)?);
    Ok(())
}

fn log(submatches: &clap::ArgMatches<'_>) -> Result<()> {
    let mut parent = Some(base::get_oid(submatches.value_of("OID").unwrap())?);
    while let Some(oid) = parent {
        let commit = base::get_commit(&oid)?;

        println!("commit {}", oid);
        println!("    {}", commit.message);
        println!("");

        parent = commit.parent;
    }
    Ok(())
}

fn checkout(submatches: &clap::ArgMatches<'_>) -> Result<()> {
    let oid = base::get_oid(submatches.value_of("OID").unwrap())?;
    let commit = base::get_commit(&oid)?;
    base::read_tree(&commit.tree)?;
    data::update_ref("HEAD", &oid)
}

fn tag(submatches: &clap::ArgMatches<'_>) -> Result<()> {
    let name = submatches.value_of("NAME").unwrap();
    let oid = base::get_oid(submatches.value_of("OID").unwrap())?;
    base::create_tag(name, &oid)
}
