use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117421699: FileType = FileType {
    file_format: &FileFormat {
        id: 117_421_699,
        source_type: SourceType::Wikidata,
        name: "JSONC",
        extensions: &["jsonc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
