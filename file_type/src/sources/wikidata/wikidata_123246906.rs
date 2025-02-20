use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123246906: FileType = FileType {
    file_format: &FileFormat {
        id: 123_246_906,
        source_type: SourceType::Wikidata,
        name: "Movie Collector Database",
        extensions: &["mvc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
