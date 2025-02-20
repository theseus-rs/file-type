use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125857400: FileType = FileType {
    file_format: &FileFormat {
        id: 125_857_400,
        source_type: SourceType::Wikidata,
        name: "JScript Encoded File",
        extensions: &["jse"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
