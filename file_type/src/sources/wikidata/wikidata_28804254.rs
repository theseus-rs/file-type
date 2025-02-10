use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28804254: FileType = FileType {
    file_format: &FileFormat {
        id: 28_804_254,
        source_type: SourceType::Wikidata,
        name: "Uniform Office Text",
        extensions: &["uot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
