use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1162461536: FileType = FileType {
    file_format: &FileFormat {
        id: 1_162_461_536,
        source_type: SourceType::Iana,
        name: "vnd.airzip.accelerator.azv",
        extensions: &[],
        media_types: &["image/vnd.airzip.accelerator.azv"],
        signatures: &[],
        related_formats: &[],
    },
};
