use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967389: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_389,
        source_type: SourceType::Wikidata,
        name: "Adlib Tracker module",
        extensions: &["sng"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
