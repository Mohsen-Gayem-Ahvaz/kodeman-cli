use crate::core::ext::ResponseExt;
use crate::core::utils::{gen_keypairs, input, temp_store};
use crate::core::{http, AppResult, BASE_ST};
use crate::user::models::{EmailVerfReq, LoginReq, LoginResp, SendCodeReq, SendCodeResp, TempRegisterReq, TempRegisterResp, VerifyEmailResp};
use crate::user::states::{TempUserState, UserState};
use colored::Colorize;


pub fn _login_user() -> AppResult<()> {

    let mut base_st = BASE_ST.lock().unwrap().clone().unwrap();
    let send_code_req = SendCodeReq {
        username: input("Username: ")?,
        email: input("Email: ")?,
        usage: String::from("key-exchange"),
    };

    let rsp = http::post_request::<SendCodeReq>(
        Some(&send_code_req), (&base_st.send_code_url()).as_str()
    )?;
    let act_rsp = rsp.handle_status("login")?.json::<SendCodeResp>()?;
    println!("\
        {} if giving credentials are valid, a verification code will send to \
        your email address", "note:".cyan()
    );

    println!("{} few seconds to go ...", "Wait".cyan());
    let [prikey, pubkey] = gen_keypairs(2048)?;
    let login_req = LoginReq {
        username: send_code_req.username,
        email: send_code_req.email,
        exp: act_rsp.exp,
        code: rpassword::prompt_password("enter Code: ")?,
        dev: input("Device Name: ")?,
        key: pubkey,
        nonce: act_rsp.nonce,
    };
    let rsp = http::post_request::<LoginReq>(
        Some(&login_req), (&base_st.login_url()).as_str()
    )?;
    let act_rsp = rsp.handle_status("login")?.json::<LoginResp>()?;

    let user_st = UserState {
        username: login_req.username,
        email: login_req.email,
        dev: login_req.dev,
        prikey,
        pubkey: login_req.key,
        service_pubkey: act_rsp.key,
        mark: vec![],
    };
    user_st.save()?;
    println!("{} saving User", "done:".green());

    base_st.default_user = user_st.username;
    base_st.save()?;
    println!(
        "{} user ({}) is now the default user",
        "note:".cyan(), base_st.default_user
    );

    Ok(())
}

pub fn _register_user() -> AppResult<()> {

    let mut base_st = BASE_ST.lock().unwrap().clone().unwrap();
    let new_user = TempRegisterReq {
        username: input("Username: ")?,
        email: input("Email: ")?,
        dev: input("Device Name: ")?,
    };
    new_user.validate()?;

    let rsp = http::post_request::<TempRegisterReq>(
        Some(&new_user), (&base_st.temp_register_url()).as_str()
    )?;
    let act_rsp = rsp.handle_status("login")?.json::<TempRegisterResp>()?;
    println!("{} first step done -> a code sent to your email", "done:".green());

    let [prikey, pubkey] = gen_keypairs(2048)?;
    let tmp_user_st = TempUserState {
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        dev: new_user.dev.clone(),
        nonce: act_rsp.nonce,
        exp: act_rsp.exp,
        pri_keypem: prikey.clone(),
        pub_keypem: pubkey.clone()
    };
    let tmp_user_st_pth = temp_store::<TempUserState>(Some(&tmp_user_st))?;

    base_st.lts_register_claims = tmp_user_st_pth;
    base_st.save()?;

    let service_pubkey = _verify_email(tmp_user_st)?;
    println!("{} email validation", "done:".green());

    let user_st = UserState {
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        dev: new_user.dev.clone(),
        prikey,
        pubkey,
        service_pubkey,
        mark: vec![],
    };
    user_st.save()?;
    println!("{} saving User", "done:".green());

    base_st.default_user = user_st.username;
    base_st.save()?;
    println!(
        "{} user ({}) is now the default user",
        "note:".cyan(), base_st.default_user
    );

    Ok(())
}


pub fn _verify_email(
    tmp_user_st: TempUserState
) -> AppResult<String> {
    let base_st = BASE_ST.lock().unwrap().clone().unwrap();

    let form = EmailVerfReq {
        username: tmp_user_st.username,
        dev: tmp_user_st.dev,
        exp: tmp_user_st.exp,
        nonce: tmp_user_st.nonce,
        code: rpassword::prompt_password("enter Code: ")?,
        pubkey: tmp_user_st.pub_keypem
    };

    let rsp = http::post_request::<EmailVerfReq>(
        Some(&form), (&base_st.verify_email_url()).as_str()
    )?;
    let act_rsp = rsp.handle_status("login")?.json::<VerifyEmailResp>()?;

    Ok(act_rsp.key)
}

