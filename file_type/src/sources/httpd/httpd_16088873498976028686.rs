use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16088873498976028686: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "octet stream",
    extensions: &[
        "bin", "dms", "lrf", "mar", "so", "dist", "distz", "pkg", "bpk", "dump", "elc", "deploy",
    ],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
