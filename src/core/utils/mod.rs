pub mod msg;

// Lib
use std::path::PathBuf;
use std::io::Write;
use std::io;
use std::fs;
use rsa::RsaPrivateKey;
use rsa::RsaPublicKey;
use rsa::pkcs8::EncodePublicKey;
use rsa::pkcs8::EncodePrivateKey;
use serde::Serialize;
use std::io::ErrorKind as IoErrorKind;
use colored::Colorize;
use crate::core::error::AppError;
use crate::core::ext::JsonExt;
use crate::core::AppResult;
use crate::core::BASE_PTH;
use crate::core::utils::msg::done;


pub fn temp_store<M>(ctx: Option<&M>) -> AppResult<PathBuf>
    where M: JsonExt + Serialize + Default
{

    let temp_dir = BASE_PTH.get().unwrap().clone().join(".temp.d");
    let mut ctn = 0;
    let mut pth: PathBuf;
    
    loop {
        pth = temp_dir.join(format!(".temp.{}.json", ctn));
        match fs::File::create_new(&pth) {
            Ok(f) => {
                if let Some(ctx) = ctx {
                    ctx.to_file(f, true)?;
                } else {
                    M::default().to_file(f, true)?;
                }
                break;
            },
            Err(e) => {
                if e.kind().eq(&IoErrorKind::AlreadyExists) {
                    ctn += 1;
                } else {
                    return AppError::from(e).cause("problem in creating temp form file").into();
                }
            },
        }
    };
    
    Ok(pth)

} 


pub fn input(msg: &str) -> AppResult<String> {
    
    let mut inp = String::new();
    print!("{}", msg);
    io::stdout().flush()?;
    io::stdin().read_line(&mut inp)?;
    if inp.ends_with('\n') {
        inp.pop();
    }
    Ok(inp)

}


pub fn gen_keypairs(bit_size: usize) -> AppResult<[String; 2]> {
    
    let pri_key = RsaPrivateKey::new(
        &mut rand::thread_rng(), bit_size
    ).unwrap(); 
    let pub_key = RsaPublicKey::from(&pri_key);
    
    Ok([
        pri_key.to_pkcs8_pem(Default::default()).unwrap().to_string(),
        pub_key.to_public_key_pem(Default::default()).unwrap()
    ])

}


pub fn io_task<F, R>(mut func: F, act: &str) -> R where F: FnMut() -> R {
    println!("{} {}", "wait:".yellow(), act);
    let r = func();
    print!("\x1B[1A");
    done(act);
    io::stdout().flush().unwrap(); r
}


pub fn cpu_task<F, R>(mut func: F, act: &str) -> R where F: FnMut() -> R {
    println!("{} {}", "wait:".yellow(), act);
    let r = func();
    print!("\x1B[1A");
    done(act);
    io::stdout().flush().unwrap(); r
}

