// Mod
pub mod extra;
// Lib


#[derive(Debug)]
pub struct AppError {
    pub operation: String,
    pub quiet: bool,
    pub cause: String,
    pub hint: String,
    pub service: String,
    pub kind: AppErrorKind,
}

#[derive(Debug)]
pub enum AppErrorKind {

    // Core
    BaseDir,
    Io,
    ProcessCall,
    ConfigParse,
    InputParse,
    Form,

    // Encrypt & Decrypt
    Rsa,
    RsaParseKey,
    RsaEncrypt,
    RsaDecrypt,

    // Project
    ProjectAlreadyExists,
    ProjectNotExists,
    ProjectNotSynced,
    InvalidProjectForm,
    BranchAlreadyExists,
    BranchNotExists,
    BranchNotFetched,
    NoDefaultBranch,
    InvalidBranchForm,
    NoRope,
    NoDefaultRope,

    // Mark
    NoMarkedDir,
    AlreadyMarkedDir,
    NoKodemanDir,

    // User
    NoUser,

    // Http
    InvalidCredentials,
    InvalidRequest,
    ResourceNotExists,
    ResourceDuplicated,
    ResponseFormat,
    Http,

    Unknown,
    ResourceTooLarge,
}

