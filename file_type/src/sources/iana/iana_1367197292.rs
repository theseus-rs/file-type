use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1367197292: FileType = FileType {
    file_format: &FileFormat {
        id: 1_367_197_292,
        source_type: SourceType::Iana,
        name: "oauth-authz-req+jwt",
        extensions: &[],
        media_types: &["application/oauth-authz-req+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
