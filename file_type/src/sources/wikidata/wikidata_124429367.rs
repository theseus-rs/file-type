use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124429367: FileType = FileType {
    file_format: &FileFormat {
        id: 124_429_367,
        source_type: SourceType::Wikidata,
        name: "Pyramix Media File",
        extensions: &["pmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
