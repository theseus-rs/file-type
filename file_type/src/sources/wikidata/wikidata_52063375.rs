use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52063375: FileType = FileType {
    file_format: &FileFormat {
        id: 52_063_375,
        source_type: SourceType::Wikidata,
        name: "StatGraphics Data File",
        extensions: &["aws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
