use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1007674456: FileType = FileType {
    file_format: &FileFormat {
        id: 1_007_674_456,
        source_type: SourceType::Iana,
        name: "vnd.sss-cod",
        extensions: &[],
        media_types: &["application/vnd.sss-cod"],
        signatures: &[],
        related_formats: &[],
    },
};
