use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123436243: FileType = FileType {
    file_format: &FileFormat {
        id: 123_436_243,
        source_type: SourceType::Wikidata,
        name: "CD Style Sheet file",
        extensions: &["cds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
