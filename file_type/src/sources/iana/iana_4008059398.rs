use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4008059398: FileType = FileType {
    file_format: &FileFormat {
        id: 4_008_059_398,
        source_type: SourceType::Iana,
        name: "mud+json",
        extensions: &[],
        media_types: &["application/mud+json"],
        signatures: &[],
        related_formats: &[],
    },
};
