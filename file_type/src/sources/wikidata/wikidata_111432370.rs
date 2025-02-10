use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111432370: FileType = FileType {
    file_format: &FileFormat {
        id: 111_432_370,
        source_type: SourceType::Wikidata,
        name: "Interface Definition Language File",
        extensions: &["idl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
