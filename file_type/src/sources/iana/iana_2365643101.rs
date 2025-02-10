use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2365643101: FileType = FileType {
    file_format: &FileFormat {
        id: 2_365_643_101,
        source_type: SourceType::Iana,
        name: "sensml+json",
        extensions: &[],
        media_types: &["application/sensml+json"],
        signatures: &[],
        related_formats: &[],
    },
};
