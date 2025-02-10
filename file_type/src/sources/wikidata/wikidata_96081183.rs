use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_96081183: FileType = FileType {
    file_format: &FileFormat {
        id: 96_081_183,
        source_type: SourceType::Wikidata,
        name: "SystemModeler model archive format",
        extensions: &["sma"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
