use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1608585000: FileType = FileType {
    file_format: &FileFormat {
        id: 1_608_585_000,
        source_type: SourceType::Iana,
        name: "vnd.valve.source.compiled-map",
        extensions: &[],
        media_types: &["model/vnd.valve.source.compiled-map"],
        signatures: &[],
        related_formats: &[],
    },
};
