use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61976139: FileType = FileType {
    file_format: &FileFormat {
        id: 61_976_139,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visual FoxPro Report",
        extensions: &["frx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
