use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1556358890: FileType = FileType {
    file_format: &FileFormat {
        id: 1_556_358_890,
        source_type: SourceType::Iana,
        name: "jxsc",
        extensions: &[],
        media_types: &["image/jxsc"],
        signatures: &[],
        related_formats: &[],
    },
};
