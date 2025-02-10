use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2284811522: FileType = FileType {
    file_format: &FileFormat {
        id: 2_284_811_522,
        source_type: SourceType::Iana,
        name: "spdx+json",
        extensions: &[],
        media_types: &["application/spdx+json"],
        signatures: &[],
        related_formats: &[],
    },
};
