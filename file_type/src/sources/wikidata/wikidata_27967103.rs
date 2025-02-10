use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967103: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_103,
        source_type: SourceType::Wikidata,
        name: "Nintendo GameCube/Wii BRSTM",
        extensions: &["brstm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
