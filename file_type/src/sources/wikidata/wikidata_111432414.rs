use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111432414: FileType = FileType {
    file_format: &FileFormat {
        id: 111_432_414,
        source_type: SourceType::Wikidata,
        name: "Lisp Program Source Code File",
        extensions: &["lsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
