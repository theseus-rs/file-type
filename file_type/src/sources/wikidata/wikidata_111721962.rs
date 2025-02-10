use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111721962: FileType = FileType {
    file_format: &FileFormat {
        id: 111_721_962,
        source_type: SourceType::Wikidata,
        name: "Fortran include file",
        extensions: &["i90", "inc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
