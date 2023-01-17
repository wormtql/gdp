

// datas:
// WeaponExcelConfigData ?x
// WeaponExcelConfigData.nameTextMapHash ?x ?y
// CHS ?y "祭礼剑"

// WeaponExcelConfigData ?x
// WeaponExcelConfigData.weaponProp ?x ?y


// AvatarExcelConfigData ?x
// CHS ?x.id ?y
//
// rule (identical ?x ?y) { ?x.id ?z && WeaponExcelConfigData.id ?u && eq ?u ?z }

// WeaponExcelConfigData.nameTextMapHash ?x ?y && CHS ?y "祭礼剑" && WeaponExcelConfigData.icon ?x ?icon && split_by "" "UI_EquipIcon_" ?iconname ?icon

use clap::Parser;
use std::env::current_dir;
use std::io::{stdout, Write};
use gdp::ast::parser::{MyParser, parse};
use gdp::file_system::cached_file_system::CachedFileSystem;
use gdp::file_system::http_file_system::HttpFileSystem;
use gdp::file_system::naive_file_system::NaiveFileSystem;
use gdp::query::generic_query::GenericQueries;
use gdp::query::query::QueryProgram;
use gdp::runtime::frame::Frame;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: String,
}

fn main() {
    // let s = "WeaponExcelConfigData.namm.id ?x 100 ?y";
    // let p = MyParser;
    // let pair = parse(s).unwrap();
    // let ast = p.parse_expression(pair);
    //
    // println!("{:?}", ast);
    // println!("{:?}", parse("WeaponExcelConfigData.namm.id ?x 100 ?y"))

    let args: Args = Args::parse();

    let path = if args.path == "." {
        current_dir().unwrap()
    } else {
        current_dir().unwrap().join(args.path)
    };
    // let fs1 = NaiveFileSystem::new(path);
    let fs1 = HttpFileSystem::new("https://genshin-data.uigf.org/d/latest/");
    let fs2 = CachedFileSystem::new(Box::new(fs1));
    let p = QueryProgram {
        generic_query: GenericQueries::default(),
        file_system: Box::new(fs2)
    };
    // WeaponExcelConfigData ?x && WeaponExcelConfigData.nameTextMapHash ?x ?y && CHS ?y "祭礼剑"
    // AvatarExcelConfigData.nameTextMapHash ?x ?y && CHS ?y "可莉"

    loop {
        print!(">>> ");
        stdout().flush();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s);
        let s = s.trim();

        if s == "q" || s == "quit" || s == "exit" || s == "exit()" || s == "quit()" {
            break;
        }

        let result = p.query(&s);
        let result = result.unwrap_or(vec![]);
        let result: Vec<_> = result.into_iter().filter(|x| x.is_resolved()).map(|x| x.to_serde_map()).collect();

        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }
}
