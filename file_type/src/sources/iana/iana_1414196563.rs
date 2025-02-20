use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1414196563: FileType = FileType {
    file_format: &FileFormat {
        id: 1_414_196_563,
        source_type: SourceType::Iana,
        name: "zip",
        extensions: &[],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
