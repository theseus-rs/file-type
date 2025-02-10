use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67126629: FileType = FileType {
    file_format: &FileFormat {
        id: 67_126_629,
        source_type: SourceType::Wikidata,
        name: "Print Artist sign file format",
        extensions: &["sgn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
