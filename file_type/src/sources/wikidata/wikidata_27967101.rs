use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967101: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_101,
        source_type: SourceType::Wikidata,
        name: "Nintendo GameCube/Wii ADP",
        extensions: &["adp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
