use crate::core::states::BaseState;


impl BaseState {
    // user urls
    pub fn send_code_url(&self) -> String {
        format!("{}/user/send-code?client_id={}", self.auth_addr, self.client_id)
    }
    pub fn temp_register_url(&self) -> String {
        format!("{}/user/register?client_id={}", self.auth_addr, self.client_id)
    }
    pub fn verify_email_url(&self) -> String {
        format!(
            "{}/user/verify-email?client_id={}",
            self.auth_addr, self.client_id
        )
    }
    pub fn login_url(&self) -> String {
        format!(
            "{}/user/login?client_id={}",
            self.auth_addr, self.client_id
        )
    }
}

