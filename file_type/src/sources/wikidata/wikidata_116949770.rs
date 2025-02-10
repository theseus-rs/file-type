use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116949770: FileType = FileType {
    file_format: &FileFormat {
        id: 116_949_770,
        source_type: SourceType::Wikidata,
        name: "Winfax File",
        extensions: &["fxs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
