use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7311459: FileType = FileType {
    file_format: &FileFormat {
        id: 7_311_459,
        source_type: SourceType::Wikidata,
        name: "Relocatable Object Module Format",
        extensions: &["obj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
