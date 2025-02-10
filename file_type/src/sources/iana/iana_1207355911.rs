use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1207355911: FileType = FileType {
    file_format: &FileFormat {
        id: 1_207_355_911,
        source_type: SourceType::Iana,
        name: "vnd.multiad.creator",
        extensions: &[],
        media_types: &["application/vnd.multiad.creator"],
        signatures: &[],
        related_formats: &[],
    },
};
