use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1475930477: FileType = FileType {
    file_format: &FileFormat {
        id: 1_475_930_477,
        source_type: SourceType::Iana,
        name: "alto-networkmap+json",
        extensions: &[],
        media_types: &["application/alto-networkmap+json"],
        signatures: &[],
        related_formats: &[],
    },
};
