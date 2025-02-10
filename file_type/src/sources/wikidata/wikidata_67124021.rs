use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67124021: FileType = FileType {
    file_format: &FileFormat {
        id: 67_124_021,
        source_type: SourceType::Wikidata,
        name: "Print Artist greeting card file format",
        extensions: &["gc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
