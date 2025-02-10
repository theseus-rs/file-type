use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3988587278: FileType = FileType {
    file_format: &FileFormat {
        id: 3_988_587_278,
        source_type: SourceType::Iana,
        name: "vnd.sealed.mpeg4",
        extensions: &[],
        media_types: &["video/vnd.sealed.mpeg4"],
        signatures: &[],
        related_formats: &[],
    },
};
