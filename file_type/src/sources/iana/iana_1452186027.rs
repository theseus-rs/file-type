use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1452186027: FileType = FileType {
    file_format: &FileFormat {
        id: 1_452_186_027,
        source_type: SourceType::Iana,
        name: "dashdelta",
        extensions: &[],
        media_types: &["application/dashdelta"],
        signatures: &[],
        related_formats: &[],
    },
};
