use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117022113: FileType = FileType {
    file_format: &FileFormat {
        id: 117_022_113,
        source_type: SourceType::Wikidata,
        name: "Garden File",
        extensions: &["grd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
