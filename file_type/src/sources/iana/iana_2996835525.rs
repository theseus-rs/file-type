use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2996835525: FileType = FileType {
    file_format: &FileFormat {
        id: 2_996_835_525,
        source_type: SourceType::Iana,
        name: "vnd.eu.kasparian.car+json",
        extensions: &[],
        media_types: &["application/vnd.eu.kasparian.car+json"],
        signatures: &[],
        related_formats: &[],
    },
};
