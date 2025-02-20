use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1187587104: FileType = FileType {
    file_format: &FileFormat {
        id: 1_187_587_104,
        source_type: SourceType::Iana,
        name: "tzif-leap",
        extensions: &[],
        media_types: &["application/tzif-leap"],
        signatures: &[],
        related_formats: &[],
    },
};
