use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205468: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_468,
        source_type: SourceType::Wikidata,
        name: "Sony Mavica 411",
        extensions: &["411"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
