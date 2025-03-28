use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1312911509: FileType = FileType {
    file_format: &FileFormat {
        id: 1_312_911_509,
        source_type: SourceType::Iana,
        name: "vnd.oipf.cspg-hexbinary",
        extensions: &[],
        media_types: &["application/vnd.oipf.cspg-hexbinary"],
        signatures: &[],
        related_formats: &[],
    },
};
