use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2985206074: FileFormat = FileFormat {
    id: 2_985_206_074,
    source_type: SourceType::Httpd,
    name: "octet stream",
    extensions: &[
        "bin", "dms", "lrf", "mar", "so", "dist", "distz", "pkg", "bpk", "dump", "elc", "deploy",
    ],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
