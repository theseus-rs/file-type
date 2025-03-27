use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125599685: FileType = FileType {
    file_format: &FileFormat {
        id: 125_599_685,
        source_type: SourceType::Wikidata,
        name: "Rollei RAW image",
        extensions: &["ia"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
