use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2985206074: FileType = FileType {
    file_format: &FileFormat {
        id: 2_985_206_074,
        source_type: SourceType::Httpd,
        name: "octet stream",
        extensions: &[
            "bin", "dms", "lrf", "mar", "so", "dist", "distz", "pkg", "bpk", "dump", "elc",
            "deploy",
        ],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
