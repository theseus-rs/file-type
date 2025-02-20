use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2212794734: FileType = FileType {
    file_format: &FileFormat {
        id: 2_212_794_734,
        source_type: SourceType::Httpd,
        name: "rig cryptonote",
        extensions: &["cryptonote"],
        media_types: &["application/vnd.rig.cryptonote"],
        signatures: &[],
        related_formats: &[],
    },
};
