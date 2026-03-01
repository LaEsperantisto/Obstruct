use crate::type_env::Type;

pub struct CompileTimeEnv {
    types: Vec<Type>,
}

impl CompileTimeEnv {
    pub(crate) fn new() -> CompileTimeEnv {
        CompileTimeEnv { types: Vec::new() }
    }
}
