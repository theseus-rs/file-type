use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1054297770: FileType = FileType {
    file_format: &FileFormat {
        id: 1_054_297_770,
        source_type: SourceType::Iana,
        name: "vnd.oma.lwm2m+json",
        extensions: &[],
        media_types: &["application/vnd.oma.lwm2m+json"],
        signatures: &[],
        related_formats: &[],
    },
};
