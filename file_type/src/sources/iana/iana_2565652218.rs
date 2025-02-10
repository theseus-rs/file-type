use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2565652218: FileType = FileType {
    file_format: &FileFormat {
        id: 2_565_652_218,
        source_type: SourceType::Iana,
        name: "vnd.previewsystems.box",
        extensions: &[],
        media_types: &["application/vnd.previewsystems.box"],
        signatures: &[],
        related_formats: &[],
    },
};
