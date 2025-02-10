use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_104821916: FileType = FileType {
    file_format: &FileFormat {
        id: 104_821_916,
        source_type: SourceType::Wikidata,
        name: "Renoise instrument",
        extensions: &["rni", "xrni"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
