use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49989019: FileType = FileType {
    file_format: &FileFormat {
        id: 49_989_019,
        source_type: SourceType::Wikidata,
        name: "Macromedia (Adobe) Director Compressed Resource file",
        extensions: &["dcr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
