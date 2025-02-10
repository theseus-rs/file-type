use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2363579130: FileType = FileType {
    file_format: &FileFormat {
        id: 2_363_579_130,
        source_type: SourceType::Iana,
        name: "vnd.dwg",
        extensions: &[],
        media_types: &["image/vnd.dwg"],
        signatures: &[],
        related_formats: &[],
    },
};
