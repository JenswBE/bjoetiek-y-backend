use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth, Role};

fn get_keycloak(public_key: &'static str, roles: Vec<Role>) -> KeycloakAuth {
    KeycloakAuth {
        detailed_responses: true,
        keycloak_oid_public_key: DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(),
        required_roles: roles,
    }
}

pub fn get_keycloak_admin(public_key: &'static str) -> KeycloakAuth {
    let admin_role = vec![Role::Realm {
        role: "admin".to_owned(),
    }];
    get_keycloak(public_key, admin_role)
}
