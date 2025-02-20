use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_255: FileType = FileType {
    file_format: &FileFormat {
        id: 255,
        source_type: SourceType::Linguist,
        name: "OCaml",
        extensions: &["eliom", "eliomi", "ml", "ml4", "mli", "mll", "mly"],
        media_types: &["text/x-ocaml"],
        signatures: &[],
        related_formats: &[],
    },
};
