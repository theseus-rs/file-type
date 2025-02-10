use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111440951: FileType = FileType {
    file_format: &FileFormat {
        id: 111_440_951,
        source_type: SourceType::Wikidata,
        name: "BASIC Source Code File",
        extensions: &["bas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
