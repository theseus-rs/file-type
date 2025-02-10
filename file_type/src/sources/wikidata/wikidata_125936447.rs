use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125936447: FileType = FileType {
    file_format: &FileFormat {
        id: 125_936_447,
        source_type: SourceType::Wikidata,
        name: "Atrac Codec File v.1",
        extensions: &["aea"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
