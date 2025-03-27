use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133503583: FileType = FileType {
    file_format: &FileFormat {
        id: 133_503_583,
        source_type: SourceType::Wikidata,
        name: "Dali low resolution file",
        extensions: &["lpk", "sd0"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
