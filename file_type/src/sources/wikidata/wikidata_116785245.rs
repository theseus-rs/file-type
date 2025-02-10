use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116785245: FileType = FileType {
    file_format: &FileFormat {
        id: 116_785_245,
        source_type: SourceType::Wikidata,
        name: "602Pro PC Suite macro",
        extensions: &["cnt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
