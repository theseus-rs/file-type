use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_357: FileFormat = FileFormat {
    id: 357,
    source_type: SourceType::Linguist,
    name: "Standard ML",
    extensions: &["fun", "ml", "sig", "sml"],
    media_types: &["text/x-ocaml"],
    internal_signatures: &[],
    related_formats: &[],
};
