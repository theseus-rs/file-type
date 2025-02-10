use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27960146: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_146,
        source_type: SourceType::Wikidata,
        name: "X2A",
        extensions: &["x2a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
