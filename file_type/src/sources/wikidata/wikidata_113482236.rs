use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113482236: FileType = FileType {
    file_format: &FileFormat {
        id: 113_482_236,
        source_type: SourceType::Wikidata,
        name: "602 Graph/Chart File 1.51",
        extensions: &["gc6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
