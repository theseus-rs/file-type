use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110039586: FileType = FileType {
    file_format: &FileFormat {
        id: 110_039_586,
        source_type: SourceType::Wikidata,
        name: "Micrografx In-A-Vision Drawing",
        extensions: &["pic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
