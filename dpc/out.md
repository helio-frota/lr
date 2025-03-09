# Duplicrabs

> [!TIP]
> Exactly the same

### 🦀 1

```rust
        if let Some(errors) = response_body.errors {
            //TODO fix query not to return error in this case
            for error in errors.clone().into_iter() {
                if error.message == "failed to locate package based on purl" {
                    return Ok(HashMap::new());
                }
            }
            return Err(Error::GraphQL(errors));
        }
```

`guac-rs/lib/src/client/semantic/spog/mod.rs`

```rust
        if let Some(errors) = response_body.errors {
            //TODO fix query not to return error in this case
            for error in errors.clone().into_iter() {
                if error.message == "failed to locate package based on purl" {
                    return Ok(HashMap::new());
                }
            }
            return Err(Error::GraphQL(errors));
        }
```

`guac-rs/lib/src/client/semantic/spog/mod.rs`

### 🦀 2

```rust
    client
        .intrinsic()
        .ingest_is_dependency(
            &pkg_a.clone().into(),
            &pkg_b.clone().into(),
            PkgMatchType::SpecificVersion,
            &IsDependencyInputSpec {
                version_range: "".to_string(),
                dependency_type: DependencyType::Direct,
                justification: "dep-justification".to_string(),
                origin: "dep-origin".to_string(),
                collector: "dep-collector".to_string(),
            },
        )
        .await?;
```

`guac-rs/lib/tests/is_dependency.rs`

```rust
    client
        .intrinsic()
        .ingest_is_dependency(
            &pkg_a.clone().into(),
            &pkg_b.clone().into(),
            PkgMatchType::SpecificVersion,
            &IsDependencyInputSpec {
                version_range: "".to_string(),
                dependency_type: DependencyType::Direct,
                justification: "dep-justification".to_string(),
                origin: "dep-origin".to_string(),
                collector: "dep-collector".to_string(),
            },
        )
        .await?;
```

`guac-rs/lib/tests/is_dependency.rs`

### 🦀 3

```rust
            $client
                .intrinsic()
                .ingest_is_dependency(
                    &pkg_a.clone().into(),
                    &pkg_b.clone().into(),
                    PkgMatchType::SpecificVersion,
                    &IsDependencyInputSpec {
                        version_range: "".to_string(),
                        dependency_type: DependencyType::Direct,
                        justification: "justification".to_string(),
                        origin: "test-origin".to_string(),
                        collector: "test-collector".to_string(),
                    },
                )
                .await?;
        };
    }
```

`guac-rs/lib/tests/path.rs`

```rust
            $client
                .intrinsic()
                .ingest_is_dependency(
                    &pkg_a.clone().into(),
                    &pkg_b.clone().into(),
                    PkgMatchType::SpecificVersion,
                    &IsDependencyInputSpec {
                        version_range: "".to_string(),
                        dependency_type: DependencyType::Direct,
                        justification: "justification".to_string(),
                        origin: "test-origin".to_string(),
                        collector: "test-collector".to_string(),
                    },
                )
                .await?;
        };
    }
```

`guac-rs/lib/tests/path.rs`

### 🦀 4

```rust
    add_dep!(client, "pkg:rpm/your-app@1.0", "pkg:rpm/log4j@1.0");
    add_dep!(client, "pkg:rpm/myapp@1.0", "pkg:rpm/component-a@1.0");
    add_dep!(client, "pkg:rpm/myapp@1.0", "pkg:rpm/component-b@1.0");
    add_dep!(client, "pkg:rpm/myapp@1.0", "pkg:rpm/component-c@1.0");
```

`guac-rs/lib/tests/path.rs`

```rust
    add_dep!(client, "pkg:rpm/your-app@1.0", "pkg:rpm/log4j@1.0");
    add_dep!(client, "pkg:rpm/myapp@1.0", "pkg:rpm/component-a@1.0");
    add_dep!(client, "pkg:rpm/myapp@1.0", "pkg:rpm/component-b@1.0");
    add_dep!(client, "pkg:rpm/myapp@1.0", "pkg:rpm/component-c@1.0");
```

`guac-rs/lib/tests/path.rs`

### 🦀 5

```rust
    add_dep!(client, "pkg:rpm/component-a@1.0", "pkg:rpm/component-c@1.0");
    add_dep!(client, "pkg:rpm/component-c@1.0", "pkg:rpm/component-d@1.0");
    add_dep!(client, "pkg:rpm/component-a@1.0", "pkg:rpm/component-d@1.0");
    add_dep!(client, "pkg:rpm/component-d@1.0", "pkg:rpm/component-e@1.0");
    add_dep!(client, "pkg:rpm/component-e@1.0", "pkg:rpm/log4j@1.0");
```

`guac-rs/lib/tests/path.rs`

```rust
    add_dep!(client, "pkg:rpm/component-a@1.0", "pkg:rpm/component-c@1.0");
    add_dep!(client, "pkg:rpm/component-c@1.0", "pkg:rpm/component-d@1.0");
    add_dep!(client, "pkg:rpm/component-a@1.0", "pkg:rpm/component-d@1.0");
    add_dep!(client, "pkg:rpm/component-d@1.0", "pkg:rpm/component-e@1.0");
    add_dep!(client, "pkg:rpm/component-e@1.0", "pkg:rpm/log4j@1.0");
```

`guac-rs/lib/tests/path.rs`

> [!WARNING]
> Almost the same

### 🦀 1

```rust
#[derive(Clone, Debug, clap::Args)]
#[command(
    rename_all_env = "SCREAMING_SNAKE_CASE",
    about = "Run the query to find all certified-good packages of the package (purl)",
    args_conflicts_with_subcommands = true
)]
pub struct CertifyGoodCommand {
    #[arg(short = 'g', long = "guac", default_value = "http://localhost:8080/query")]
    pub(crate) guac_url: String,
```

`guac-rs/cli/src/certify.rs`

```rust
#[derive(Clone, Debug, clap::Args)]
#[command(
    rename_all_env = "SCREAMING_SNAKE_CASE",
    about = "Run the query to find all certified-good packages of the package (purl)",
    args_conflicts_with_subcommands = true
)]
pub struct CertifyBadCommand {
    #[arg(short = 'g', long = "guac", default_value = "http://localhost:8080/query")]
    pub(crate) guac_url: String,
```

`guac-rs/cli/src/certify.rs`

### 🦀 2

```rust
impl From<&PkgInputSpec> for ingest_certify_bad::PkgInputSpec {
    fn from(value: &PkgInputSpec) -> Self {
        Self {
            type_: value.r#type.clone(),
            namespace: value.namespace.clone(),
            name: value.name.clone(),
            version: value.version.clone(),
            qualifiers: value.qualifiers.as_ref().map(|e| e.iter().map(|e| e.into()).collect()),
            subpath: value.subpath.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/intrinsic/certify_bad/ingest.rs`

```rust
impl From<&PkgInputSpec> for ingest_certify_good::PkgInputSpec {
    fn from(value: &PkgInputSpec) -> Self {
        Self {
            type_: value.r#type.clone(),
            namespace: value.namespace.clone(),
            name: value.name.clone(),
            version: value.version.clone(),
            qualifiers: value.qualifiers.as_ref().map(|e| e.iter().map(|e| e.into()).collect()),
            subpath: value.subpath.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/intrinsic/certify_good/ingest.rs`

### 🦀 3

```rust
impl From<&PkgSpec> for query_certify_bad::PkgSpec {
    fn from(value: &PkgSpec) -> Self {
        Self {
            id: value.id.clone(),
            type_: value.r#type.clone(),
            namespace: value.namespace.clone(),
            name: value.name.clone(),
            version: value.version.clone(),
            qualifiers: value
                .qualifiers
                .as_ref()
                .map(|inner| inner.iter().map(|e| e.into()).collect()),
            match_only_empty_qualifiers: value.match_only_empty_qualifiers,
            subpath: value.subpath.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/intrinsic/certify_bad/query.rs`

```rust
impl From<&PkgSpec> for query_certify_good::PkgSpec {
    fn from(value: &PkgSpec) -> Self {
        Self {
            id: value.id.clone(),
            type_: value.r#type.clone(),
            namespace: value.namespace.clone(),
            name: value.name.clone(),
            version: value.version.clone(),
            qualifiers: value
                .qualifiers
                .as_ref()
                .map(|inner| inner.iter().map(|e| e.into()).collect()),
            match_only_empty_qualifiers: value.match_only_empty_qualifiers,
            subpath: value.subpath.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/intrinsic/certify_good/query.rs`

### 🦀 4

```rust
impl From<&PkgSpec> for query_certify_bad::PkgSpec {
    fn from(value: &PkgSpec) -> Self {
        Self {
            id: value.id.clone(),
            type_: value.r#type.clone(),
            namespace: value.namespace.clone(),
            name: value.name.clone(),
            version: value.version.clone(),
            qualifiers: value
                .qualifiers
                .as_ref()
                .map(|inner| inner.iter().map(|e| e.into()).collect()),
            match_only_empty_qualifiers: value.match_only_empty_qualifiers,
            subpath: value.subpath.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/intrinsic/certify_bad/query.rs`

```rust
impl From<&PkgSpec> for query_certify_vuln::PkgSpec {
    fn from(value: &PkgSpec) -> Self {
        Self {
            id: value.id.clone(),
            type_: value.r#type.clone(),
            namespace: value.namespace.clone(),
            name: value.name.clone(),
            version: value.version.clone(),
            qualifiers: value
                .qualifiers
                .as_ref()
                .map(|inner| inner.iter().map(|e| e.into()).collect()),
            match_only_empty_qualifiers: value.match_only_empty_qualifiers,
            subpath: value.subpath.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/intrinsic/certify_vuln/query.rs`

### 🦀 5

```rust
impl From<&allCertifyVEXStatementTree> for CertifyVexStatement {
    fn from(value: &allCertifyVEXStatementTree) -> Self {
        Self {
            id: value.id.clone(),
            subject: PackageOrArtifact::from(&value.subject),
            vulnerability: Vulnerability::from(&value.vulnerability),
            status: VexStatus::from(&value.status),
            vex_justification: VexJustification::from(&value.vex_justification),
            statement: value.statement.clone(),
            status_notes: value.status_notes.clone(),
            known_since: value.known_since,
            origin: value.origin.clone(),
            collector: value.collector.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability.rs`

```rust
impl From<&allCertifyVEXStatementTree> for CertifyVexStatement {
    fn from(value: &allCertifyVEXStatementTree) -> Self {
        Self {
            id: value.id.clone(),
            subject: PackageOrArtifact::from(&value.subject),
            vulnerability: Vulnerability::from(&value.vulnerability),
            status: VexStatus::from(&value.status),
            vex_justification: VexJustification::from(&value.vex_justification),
            statement: value.statement.clone(),
            status_notes: value.status_notes.clone(),
            known_since: value.known_since,
            origin: value.origin.clone(),
            collector: value.collector.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability_by_sbom_uri.rs`

### 🦀 6

```rust
impl From<&allCertifyVEXStatementTree> for CertifyVexStatement {
    fn from(value: &allCertifyVEXStatementTree) -> Self {
        Self {
            id: value.id.clone(),
            subject: PackageOrArtifact::from(&value.subject),
            vulnerability: Vulnerability::from(&value.vulnerability),
            status: VexStatus::from(&value.status),
            vex_justification: VexJustification::from(&value.vex_justification),
            statement: value.statement.clone(),
            status_notes: value.status_notes.clone(),
            known_since: value.known_since,
            origin: value.origin.clone(),
            collector: value.collector.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability.rs`

```rust
impl From<&allCertifyVEXStatementTree> for CertifyVexStatement {
    fn from(value: &allCertifyVEXStatementTree) -> Self {
        Self {
            id: value.id.clone(),
            subject: PackageOrArtifact::from(&value.subject),
            vulnerability: Vulnerability::from(&value.vulnerability),
            status: VexStatus::from(&value.status),
            vex_justification: VexJustification::from(&value.vex_justification),
            statement: value.statement.clone(),
            status_notes: value.status_notes.clone(),
            known_since: value.known_since,
            origin: value.origin.clone(),
            collector: value.collector.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/query.rs`

### 🦀 7

```rust
impl From<&FVVexStatus> for VexStatus {
    fn from(value: &FVVexStatus) -> Self {
        match value {
            FVVexStatus::NOT_AFFECTED => Self::NotAffected,
            FVVexStatus::AFFECTED => Self::Affected,
            FVVexStatus::FIXED => Self::Fixed,
            FVVexStatus::UNDER_INVESTIGATION => Self::UnderInvestigation,
            FVVexStatus::Other(inner) => Self::Other(inner.clone()),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability.rs`

```rust
impl From<&FVVexStatus> for VexStatus {
    fn from(value: &FVVexStatus) -> Self {
        match value {
            FVVexStatus::NOT_AFFECTED => Self::NotAffected,
            FVVexStatus::AFFECTED => Self::Affected,
            FVVexStatus::FIXED => Self::Fixed,
            FVVexStatus::UNDER_INVESTIGATION => Self::UnderInvestigation,
            FVVexStatus::Other(inner) => Self::Other(inner.clone()),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability_by_sbom_uri.rs`

### 🦀 8

```rust
impl From<&AllCertifyVexStatementTreeSubject> for PackageOrArtifact {
    fn from(value: &AllCertifyVexStatementTreeSubject) -> Self {
        match value {
            AllCertifyVexStatementTreeSubject::Package(inner) => Self::Package(inner.into()),
            AllCertifyVexStatementTreeSubject::Artifact(inner) => {
                todo!("artifact not implemented")
            }
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability.rs`

```rust
impl From<&AllCertifyVexStatementTreeSubject> for PackageOrArtifact {
    fn from(value: &AllCertifyVexStatementTreeSubject) -> Self {
        match value {
            AllCertifyVexStatementTreeSubject::Package(inner) => Self::Package(inner.into()),
            AllCertifyVexStatementTreeSubject::Artifact(inner) => {
                todo!("artifact not implemented")
            }
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability_by_sbom_uri.rs`

### 🦀 9

```rust
impl From<&AllCertifyVexStatementTreeSubjectOnPackageNamespacesNamesVersions> for PackageVersion {
    fn from(value: &AllCertifyVexStatementTreeSubjectOnPackageNamespacesNamesVersions) -> Self {
        Self {
            id: value.id.clone(),
            version: value.version.clone(),
            qualifiers: value.qualifiers.iter().map(|e| e.into()).collect(),
            subpath: value.subpath.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability.rs`

```rust
impl From<&AllCertifyVexStatementTreeSubjectOnPackageNamespacesNamesVersions> for PackageVersion {
    fn from(value: &AllCertifyVexStatementTreeSubjectOnPackageNamespacesNamesVersions) -> Self {
        Self {
            id: value.id.clone(),
            version: value.version.clone(),
            qualifiers: value.qualifiers.iter().map(|e| e.into()).collect(),
            subpath: value.subpath.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability_by_sbom_uri.rs`

### 🦀 10

```rust
impl From<&FVVexJustification> for VexJustification {
    fn from(value: &FVVexJustification) -> Self {
        match value {
            FVVexJustification::COMPONENT_NOT_PRESENT => Self::ComponentNotPresent,
            FVVexJustification::VULNERABLE_CODE_NOT_PRESENT => Self::VulnerableCodeNotPresent,
            FVVexJustification::VULNERABLE_CODE_NOT_IN_EXECUTE_PATH => Self::VulnerableCodeNotInExecutePath,
            FVVexJustification::VULNERABLE_CODE_CANNOT_BE_CONTROLLED_BY_ADVERSARY => {
                Self::VulnerableCodeCannotBeControlledByAdversary
            }
            FVVexJustification::INLINE_MITIGATIONS_ALREADY_EXIST => Self::InlineMitigationsAlreadyExist,
            FVVexJustification::NOT_PROVIDED => Self::NotProvided,
            FVVexJustification::Other(inner) => Self::Other(inner.clone()),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability.rs`

```rust
impl From<&FVVexJustification> for VexJustification {
    fn from(value: &FVVexJustification) -> Self {
        match value {
            FVVexJustification::COMPONENT_NOT_PRESENT => Self::ComponentNotPresent,
            FVVexJustification::VULNERABLE_CODE_NOT_PRESENT => Self::VulnerableCodeNotPresent,
            FVVexJustification::VULNERABLE_CODE_NOT_IN_EXECUTE_PATH => Self::VulnerableCodeNotInExecutePath,
            FVVexJustification::VULNERABLE_CODE_CANNOT_BE_CONTROLLED_BY_ADVERSARY => {
                Self::VulnerableCodeCannotBeControlledByAdversary
            }
            FVVexJustification::INLINE_MITIGATIONS_ALREADY_EXIST => Self::InlineMitigationsAlreadyExist,
            FVVexJustification::NOT_PROVIDED => Self::NotProvided,
            FVVexJustification::Other(inner) => Self::Other(inner.clone()),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability_by_sbom_uri.rs`

### 🦀 11

```rust
impl From<&AllCertifyVulnTreeMetadata> for ScanMetadata {
    fn from(value: &AllCertifyVulnTreeMetadata) -> Self {
        Self {
            db_uri: value.db_uri.clone(),
            db_version: value.db_version.clone(),
            scanner_uri: value.scanner_uri.clone(),
            scanner_version: value.scanner_version.clone(),
            time_scanned: value.time_scanned,
            origin: value.origin.clone(),
            collector: value.collector.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability.rs`

```rust
impl From<&AllCertifyVulnTreeMetadata> for ScanMetadata {
    fn from(value: &AllCertifyVulnTreeMetadata) -> Self {
        Self {
            db_uri: value.db_uri.clone(),
            db_version: value.db_version.clone(),
            scanner_uri: value.scanner_uri.clone(),
            scanner_version: value.scanner_version.clone(),
            time_scanned: value.time_scanned,
            origin: value.origin.clone(),
            collector: value.collector.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability_by_sbom_uri.rs`

### 🦀 12

```rust
impl From<&AllCertifyVulnTreePackageNamespacesNamesVersions> for PackageVersion {
    fn from(value: &AllCertifyVulnTreePackageNamespacesNamesVersions) -> Self {
        Self {
            id: value.id.clone(),
            version: value.version.clone(),
            qualifiers: value.qualifiers.iter().map(|e| e.into()).collect(),
            subpath: value.subpath.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability.rs`

```rust
impl From<&AllCertifyVulnTreePackageNamespacesNamesVersions> for PackageVersion {
    fn from(value: &AllCertifyVulnTreePackageNamespacesNamesVersions) -> Self {
        Self {
            id: value.id.clone(),
            version: value.version.clone(),
            qualifiers: value.qualifiers.iter().map(|e| e.into()).collect(),
            subpath: value.subpath.clone(),
        }
    }
}
```

`guac-rs/lib/src/client/semantic/spog/find_vulnerability_by_sbom_uri.rs`

### 🦀 13

```rust
    let dep_a_b_id = client
        .intrinsic()
        .ingest_is_dependency(
            &pkg_a.clone().into(),
            &pkg_b.clone().into(),
            PkgMatchType::SpecificVersion,
            &IsDependencyInputSpec {
                version_range: "".to_string(),
                dependency_type: DependencyType::Direct,
                justification: "a-b justification".to_string(),
                origin: "test-origin".to_string(),
                collector: "test-collector".to_string(),
            },
        )
        .await?;
```

`guac-rs/lib/tests/path.rs`

```rust
    let dep_a_c_id = client
        .intrinsic()
        .ingest_is_dependency(
            &pkg_a.clone().into(),
            &pkg_c.clone().into(),
            PkgMatchType::SpecificVersion,
            &IsDependencyInputSpec {
                version_range: "".to_string(),
                dependency_type: DependencyType::Direct,
                justification: "a-c justification".to_string(),
                origin: "test-origin".to_string(),
                collector: "test-collector".to_string(),
            },
        )
        .await?;
```

`guac-rs/lib/tests/path.rs`

### 🦀 14

```rust
    let _vuln_a = client
        .intrinsic()
        .ingest_certify_vuln(
            &pkg_a.clone().into(),
            &VulnerabilityInputSpec {
                r#type: "osv".to_string(),
                vulnerability_id: "ghsa-vuln-a".to_string(),
            },
            &ScanMetadata {
                db_uri: "test-db-uri".to_string(),
                db_version: "test-db-version".to_string(),
                scanner_uri: "test-scanner-uri".to_string(),
                scanner_version: "test-scanner-version".to_string(),
                time_scanned: Default::default(),
                origin: "test-vuln-origin".to_string(),
                collector: "test-vuln-collector".to_string(),
            },
        )
        .await?;
```

`guac-rs/lib/tests/path.rs`

```rust
    let _vuln_d = client
        .intrinsic()
        .ingest_certify_vuln(
            &pkg_d.clone().into(),
            &VulnerabilityInputSpec {
                r#type: "osv".to_string(),
                vulnerability_id: "ghsa-vuln-d".to_string(),
            },
            &ScanMetadata {
                db_uri: "test-db-uri".to_string(),
                db_version: "test-db-version".to_string(),
                scanner_uri: "test-scanner-uri".to_string(),
                scanner_version: "test-scanner-version".to_string(),
                time_scanned: Default::default(),
                origin: "test-vuln-origin".to_string(),
                collector: "test-vuln-collector".to_string(),
            },
        )
        .await?;
```

`guac-rs/lib/tests/path.rs`

### 🦀 15

```rust
//TODO do proper testing
// ./bin/guacone collect files --gql-addr http://localhost:8085/query ./rhel-7.9.z.json
// ./bin/guacone collect files --gql-addr http://localhost:8085/query ./cve-2022-2284.json
#[ignore]
#[tokio::test]
async fn find_vulnerability() -> Result<(), anyhow::Error> {
    let client = GuacClient::new("http://localhost:8085/query");
```

`guac-rs/lib/tests/spog.rs`

```rust
//TODO do proper testing
// ./bin/guacone collect files --gql-addr http://localhost:8085/query ./rhel-7.9.z.json
// ./bin/guacone collect files --gql-addr http://localhost:8085/query ./cve-2022-2284.json
#[ignore]
#[tokio::test]
async fn find_vulnerability_by_sbom_uri() -> Result<(), anyhow::Error> {
    let client = GuacClient::new("http://localhost:8085/query");
```

`guac-rs/lib/tests/spog.rs`

### 🦀 16

```rust
    client
        .intrinsic()
        .ingest_vuln_equal(
            &vuln_ghsa,
            &vuln_cve,
            &VulnEqualInputSpec {
                justification: "test-justification".to_string(),
                origin: "test-origin".to_string(),
                collector: "test-collector".to_string(),
            },
        )
        .await?;
```

`guac-rs/lib/tests/vuln_equal.rs`

```rust
    client
        .intrinsic()
        .ingest_vuln_equal(
            &vuln_osv,
            &vuln_cve,
            &VulnEqualInputSpec {
                justification: "test-justification".to_string(),
                origin: "test-origin".to_string(),
                collector: "test-collector".to_string(),
            },
        )
        .await?;
```

`guac-rs/lib/tests/vuln_equal.rs`

### 🦀 17

```rust
    let _found_vulns = client
        .intrinsic()
        .vuln_equal(&VulnEqualSpec {
            vulnerabilities: Some(vec![VulnerabilitySpec {
                vulnerability_id: Some("OSV-2021-45046".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        })
        .await?;
```

`guac-rs/lib/tests/vuln_equal.rs`

```rust
    let _found_vulns = client
        .intrinsic()
        .vuln_equal(&VulnEqualSpec {
            vulnerabilities: Some(vec![VulnerabilitySpec {
                vulnerability_id: Some("CVE-2021-45046".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        })
        .await?;
```

`guac-rs/lib/tests/vuln_equal.rs`

### 🦀 18

```rust
    let _result = client
        .intrinsic()
        .ingest_vulnerability(&VulnerabilityInputSpec {
            r#type: "NOT-test-vuln".to_string(),
            vulnerability_id: "ghsa-osv-cve-44".to_string(),
        })
        .await?;
```

`guac-rs/lib/tests/vulnerability.rs`

```rust
    let _result = client
        .intrinsic()
        .ingest_vulnerability(&VulnerabilityInputSpec {
            r#type: "test-vuln".to_string(),
            vulnerability_id: "ghsa-osv-cve-44".to_string(),
        })
        .await?;
```

`guac-rs/lib/tests/vulnerability.rs`


