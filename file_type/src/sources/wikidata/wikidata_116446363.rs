use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116446363: FileType = FileType {
    file_format: &FileFormat {
        id: 116_446_363,
        source_type: SourceType::Wikidata,
        name: "Work File",
        extensions: &["w"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
