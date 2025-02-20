use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67124713: FileType = FileType {
    file_format: &FileFormat {
        id: 67_124_713,
        source_type: SourceType::Wikidata,
        name: "Print Artist postcard file format",
        extensions: &["pc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
