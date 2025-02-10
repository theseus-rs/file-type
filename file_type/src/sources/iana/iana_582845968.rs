use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_582845968: FileType = FileType {
    file_format: &FileFormat {
        id: 582_845_968,
        source_type: SourceType::Iana,
        name: "H265",
        extensions: &[],
        media_types: &["video/H265"],
        signatures: &[],
        related_formats: &[],
    },
};
