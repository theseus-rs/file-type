use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2838883210: FileType = FileType {
    file_format: &FileFormat {
        id: 2_838_883_210,
        source_type: SourceType::Iana,
        name: "CALS-1840",
        extensions: &[],
        media_types: &["application/CALS-1840"],
        signatures: &[],
        related_formats: &[],
    },
};
