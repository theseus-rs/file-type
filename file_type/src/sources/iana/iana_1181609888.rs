use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1181609888: FileType = FileType {
    file_format: &FileFormat {
        id: 1_181_609_888,
        source_type: SourceType::Iana,
        name: "senml+json",
        extensions: &[],
        media_types: &["application/senml+json"],
        signatures: &[],
        related_formats: &[],
    },
};
