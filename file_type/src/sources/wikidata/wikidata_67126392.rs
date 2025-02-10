use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67126392: FileType = FileType {
    file_format: &FileFormat {
        id: 67_126_392,
        source_type: SourceType::Wikidata,
        name: "Print Artist quote file format",
        extensions: &["qot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
