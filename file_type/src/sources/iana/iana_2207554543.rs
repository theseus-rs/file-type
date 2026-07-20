use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2207554543: FileType = FileType {
    file_format: &FileFormat {
        id: 2_207_554_543,
        source_type: SourceType::Iana,
        name: "vnd.digitalstack.document+zip",
        extensions: &[],
        media_types: &["application/vnd.digitalstack.document+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
