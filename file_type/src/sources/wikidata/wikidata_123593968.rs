use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123593968: FileType = FileType {
    file_format: &FileFormat {
        id: 123_593_968,
        source_type: SourceType::Wikidata,
        name: "SGI Movie File",
        extensions: &["movie", "mv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
