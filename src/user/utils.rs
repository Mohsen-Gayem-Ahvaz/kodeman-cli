use crate::core::error::AppErrorKind;
use crate::core::ext::JsonExt;
use crate::core::{AppResult, BASE_PTH, BASE_ST};
use crate::user::states::UserState;


pub fn _get_default_user() -> AppResult<UserState> {

    let base_st = BASE_ST.lock().unwrap().clone().unwrap();
    if base_st.default_user.is_empty() {
        return AppErrorKind::NoUser.cause("no default user founded").into();
    }
    let user_conf_pth = BASE_PTH.get().unwrap()
        .join("user.json.d")
        .join(format!("{}.json", base_st.default_user));

    let user_st = UserState::from_path(&user_conf_pth).map_err(|e| {
        e.operation("finding user")
            .cause(format!("user {} not exists in system", base_st.default_user))
    })?;

    Ok(user_st)

}


pub fn _get_user(username: &str) -> AppResult<UserState> {

    let user_conf_pth = BASE_PTH.get().unwrap()
        .join("user.json.d")
        .join(format!("{}.json", username));

    let user_st = UserState::from_path(&user_conf_pth).map_err(|e| {
        e.operation("finding user")
            .cause("user not exists in system")
    })?;

    Ok(user_st)

}