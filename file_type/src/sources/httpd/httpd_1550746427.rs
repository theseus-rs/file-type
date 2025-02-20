use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1550746427: FileType = FileType {
    file_format: &FileFormat {
        id: 1_550_746_427,
        source_type: SourceType::Httpd,
        name: "pkixcmp",
        extensions: &["pki"],
        media_types: &["application/pkixcmp"],
        signatures: &[],
        related_formats: &[],
    },
};
