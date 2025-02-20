use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126951749: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_749,
        source_type: SourceType::Wikidata,
        name: "OCaml source code file",
        extensions: &["ml"],
        media_types: &["text/x-ocaml"],
        signatures: &[],
        related_formats: &[],
    },
};
