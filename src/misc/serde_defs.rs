use docker_api::opts::RegistryAuth;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum RegistryAuthDef {
    Password {
        username: String,
        password: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,

        #[serde(rename = "serveraddress")]
        #[serde(skip_serializing_if = "Option::is_none")]
        server_address: Option<String>,
    },
    Token {
        #[serde(rename = "identitytoken")]
        identity_token: String,
    },
}

impl Into<RegistryAuth> for RegistryAuthDef {
    fn into(self) -> RegistryAuth {
        match self {
            Self::Password {
                username,
                password,
                email,
                server_address,
            } => {
                let mut builder = RegistryAuth::builder()
                    .username(username)
                    .password(password);

                if let Some(email) = email {
                    builder = builder.email(email);
                }

                if let Some(server_address) = server_address {
                    builder = builder.server_address(server_address);
                }

                builder.build()
            }
            Self::Token { identity_token } => RegistryAuth::token(identity_token),
        }
    }
}
