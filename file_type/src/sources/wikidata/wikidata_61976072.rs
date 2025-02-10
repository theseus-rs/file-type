use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61976072: FileType = FileType {
    file_format: &FileFormat {
        id: 61_976_072,
        source_type: SourceType::Wikidata,
        name: "FoxPro Report",
        extensions: &["frx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
