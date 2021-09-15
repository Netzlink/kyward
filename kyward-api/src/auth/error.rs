use anyhow::Error;

/// Struct representing an error in auth
#[derive(Debug)]
pub struct AuthenticationError(Error);

#[allow(non_snake_case)]
pub fn NewAuthenticationError(err: Error) -> AuthenticationError {
    AuthenticationError(err)
}