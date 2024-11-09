
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 84,
                        patch: 0,
                        pre: vec![semver::Identifier::AlphaNumeric("nightly".to_owned()), ],
                        build: vec![],
                    },
                    host: "x86_64-unknown-linux-gnu".to_owned(),
                    short_version_string: "rustc 1.84.0-nightly (4f2f477fd 2024-10-23)".to_owned(),
                    commit_hash: Some("4f2f477fded0a47b21ed3f6aeddeafa5db8bf518".to_owned()),
                    commit_date: Some("2024-10-23".to_owned()),
                    build_date: None,
                    channel: Channel::Nightly,
                }
            }
            