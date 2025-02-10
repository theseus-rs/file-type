use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52425808: FileType = FileType {
    file_format: &FileFormat {
        id: 52_425_808,
        source_type: SourceType::Wikidata,
        name: "Vista Pro Graphics",
        extensions: &["dem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
