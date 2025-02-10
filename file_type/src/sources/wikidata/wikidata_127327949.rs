use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127327949: FileType = FileType {
    file_format: &FileFormat {
        id: 127_327_949,
        source_type: SourceType::Wikidata,
        name: "Coffeescript file",
        extensions: &["coffee"],
        media_types: &["text/coffeescript"],
        signatures: &[],
        related_formats: &[],
    },
};
