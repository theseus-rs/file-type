use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_283: FileFormat = FileFormat {
    id: 283,
    source_type: SourceType::Linguist,
    name: "Raku",
    extensions: &[
        "6pl", "6pm", "nqp", "p6", "p6l", "p6m", "pl", "pl6", "pm", "pm6", "raku", "rakumod", "t",
    ],
    media_types: &["text/x-perl"],
    signatures: &[],
    related_formats: &[],
};
