use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3539317545: FileType = FileType {
    file_format: &FileFormat {
        id: 3_539_317_545,
        source_type: SourceType::Iana,
        name: "atsc-rdt+json",
        extensions: &[],
        media_types: &["application/atsc-rdt+json"],
        signatures: &[],
        related_formats: &[],
    },
};
