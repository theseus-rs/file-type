use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125857184: FileType = FileType {
    file_format: &FileFormat {
        id: 125_857_184,
        source_type: SourceType::Wikidata,
        name: "C-- source code file",
        extensions: &["c--"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
