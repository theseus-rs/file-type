use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2166384575: FileType = FileType {
    file_format: &FileFormat {
        id: 2_166_384_575,
        source_type: SourceType::Iana,
        name: "xml",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
