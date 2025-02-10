use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_910732762: FileType = FileType {
    file_format: &FileFormat {
        id: 910_732_762,
        source_type: SourceType::Iana,
        name: "vnd.kidspiration",
        extensions: &[],
        media_types: &["application/vnd.kidspiration"],
        signatures: &[],
        related_formats: &[],
    },
};
