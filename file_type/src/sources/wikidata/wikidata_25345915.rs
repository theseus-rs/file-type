use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_25345915: FileType = FileType {
    file_format: &FileFormat {
        id: 25_345_915,
        source_type: SourceType::Wikidata,
        name: "Scratch Project SB",
        extensions: &["sb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
