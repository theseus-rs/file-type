use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47896997: FileType = FileType {
    file_format: &FileFormat {
        id: 47_896_997,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange Format Style Extract",
        extensions: &["dxx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
