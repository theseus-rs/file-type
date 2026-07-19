use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1960880426: FileType = FileType {
    file_format: &FileFormat {
        id: 1_960_880_426,
        source_type: SourceType::Iana,
        name: "client-authentication+jwt",
        extensions: &[],
        media_types: &["application/client-authentication+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
