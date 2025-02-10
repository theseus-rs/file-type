use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1637610671: FileType = FileType {
    file_format: &FileFormat {
        id: 1_637_610_671,
        source_type: SourceType::Iana,
        name: "MPA",
        extensions: &[],
        media_types: &["audio/MPA"],
        signatures: &[],
        related_formats: &[],
    },
};
