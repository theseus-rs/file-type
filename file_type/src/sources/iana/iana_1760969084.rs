use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1760969084: FileType = FileType {
    file_format: &FileFormat {
        id: 1_760_969_084,
        source_type: SourceType::Iana,
        name: "vnd.deut+json",
        extensions: &[],
        media_types: &["application/vnd.deut+json"],
        signatures: &[],
        related_formats: &[],
    },
};
