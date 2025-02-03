use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_829207807: FileFormat = FileFormat {
    id: 829_207_807,
    source_type: SourceType::Linguist,
    name: "CameLIGO",
    extensions: &["mligo"],
    media_types: &["text/x-ocaml"],
    internal_signatures: &[],
    related_formats: &[],
};
