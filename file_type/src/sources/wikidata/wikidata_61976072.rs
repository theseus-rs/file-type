use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
