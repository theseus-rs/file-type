use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5010021: FileType = FileType {
    file_format: &FileFormat {
        id: 5_010_021,
        source_type: SourceType::Wikidata,
        name: "CDX Format",
        extensions: &["cdx"],
        media_types: &["chemical/x-cdx"],
        signatures: &[],
        related_formats: &[],
    },
};
