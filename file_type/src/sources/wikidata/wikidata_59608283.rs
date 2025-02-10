use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59608283: FileType = FileType {
    file_format: &FileFormat {
        id: 59_608_283,
        source_type: SourceType::Wikidata,
        name: "KryoFlux 2 format",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
