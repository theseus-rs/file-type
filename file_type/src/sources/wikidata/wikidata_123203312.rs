use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123203312: FileType = FileType {
    file_format: &FileFormat {
        id: 123_203_312,
        source_type: SourceType::Wikidata,
        name: "TiVo Video File",
        extensions: &["tivo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
