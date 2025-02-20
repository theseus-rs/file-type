use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000561: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_561,
        source_type: SourceType::Wikidata,
        name: "Kryoflux Stream",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
