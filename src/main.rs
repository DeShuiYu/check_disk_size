use std::{fs, io};
use std::path::{Path, PathBuf};
use std::string::String;
use clap::Parser;
use humansize::{WINDOWS};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use prettytable::{Cell, Row};
use walkdir::WalkDir;

fn get_size_with_dir(dir_path:&Path)->Result<u64,io::Error>{
    let mut all_size = 0u64;
    for dir_entry in WalkDir::new(dir_path){
        match dir_entry {
            Ok(entry)=>{
                if entry.file_type().is_file(){
                    all_size += entry.metadata().unwrap().len();
                }
            },
            Err(e)=>{
                eprintln!("{:?}",e);
            }
        }
    }
    Ok(all_size)
}

#[allow(dead_code)]
fn send_mail(from_mail:&str,to_mail:&str,content:String){
    let local_time = chrono::prelude::Local::now();
    let email = Message::builder()
        .from(from_mail.parse().unwrap())
        .to(to_mail.parse().unwrap())
        .subject(format!("{:?} 检查磁盘情况报告",local_time.format("%Y-%m-%d")))
        .body(content)
        .expect("failed to create message");

    // SMTP服务器配置
    let credentials = Credentials::new("zhipengtu@zju.edu.cn".to_string(), "WSKcY48D4MIiAHCJ".to_string());
    let mailer = SmtpTransport::relay("smtp.zju.edu.cn")
        .unwrap()
        .credentials(credentials)
        .build();
    // 发送邮件
   mailer.send(&email).expect("Failed to send mail");
}

#[derive(Parser,Debug)]
struct Args{
    #[arg(short='d',long="dir",required = true)]
    dir_path:PathBuf,
}

fn main(){
    let args = Args::parse();
    let mut table = prettytable::Table::new();
    table.add_row(Row::new(vec![
        Cell::new("文件路径").style_spec("c"),
        Cell::new("占用大小").style_spec("c")
    ]));
    println!("args:{:?}",args);
    for direntry in fs::read_dir(args.dir_path.as_path()).expect("read dir error"){
        match direntry {
            Ok(entry)=>{
                if let Ok(size) = get_size_with_dir(&entry.path()){
                    table.add_row(Row::new(vec![
                        Cell::new(&format!("{:?}",entry.path())).style_spec("l"),
                        Cell::new(&humansize::format_size(size,WINDOWS)).style_spec("c")
                    ]));
                }
            },
            Err(e)=>{
                eprintln!("{:?}",e);
            }
        }
    }
    table.printstd();
}
