use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130372707: FileType = FileType {
    file_format: &FileFormat {
        id: 130_372_707,
        source_type: SourceType::Wikidata,
        name: "newLISP source code file",
        extensions: &["kif", "lsp", "nl"],
        media_types: &["application/x-newlisp", "text/x-newlisp"],
        signatures: &[],
        related_formats: &[],
    },
};
