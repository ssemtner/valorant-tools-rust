use anyhow::Result;

use rustls::ClientConfig;

pub fn create_tls_config() -> Result<ClientConfig> {
    let mut root_store = rustls::RootCertStore::empty();

    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let cipher_suites = lookup_suites(CIPHER_SUITES);

    let config = ClientConfig::builder()
        .with_cipher_suites(&cipher_suites)
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .expect("Failed to build TLS config")
        .with_root_certificates(root_store)
        .with_no_client_auth();

    Ok(config)
}

const CIPHER_SUITES: &'static [&str] = &[
    "TLS13_CHACHA20_POLY1305_SHA256",
    "TLS13_AES_128_GCM_SHA256",
    "TLS13_AES_256_GCM_SHA384",
    "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
];

fn find_suite(name: &str) -> Option<rustls::SupportedCipherSuite> {
    for suite in rustls::ALL_CIPHER_SUITES {
        let sname = format!("{:?}", suite.suite()).to_lowercase();

        if sname == name.to_string().to_lowercase() {
            return Some(*suite);
        }
    }

    None
}

fn lookup_suites(suites: &[&str]) -> Vec<rustls::SupportedCipherSuite> {
    let mut out = Vec::new();

    for csname in suites {
        let scs = find_suite(csname);
        match scs {
            Some(s) => out.push(s),
            None => panic!("cannot look up ciphersuite '{}'", csname),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_suites() {
        let suites = lookup_suites(CIPHER_SUITES);

        assert_eq!(suites.len(), 4);
    }

    #[test]
    fn test_find_suite() {
        let suite = find_suite("TLS13_CHACHA20_POLY1305_SHA256");

        assert!(suite.is_some());
    }

    #[test]
    fn test_create_tls_config() {
        let config = create_tls_config();

        assert!(config.is_ok());
    }
}