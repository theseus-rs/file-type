use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113376526: FileType = FileType {
    file_format: &FileFormat {
        id: 113_376_526,
        source_type: SourceType::Wikidata,
        name: "RED Thumbnail File",
        extensions: &["rtn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
