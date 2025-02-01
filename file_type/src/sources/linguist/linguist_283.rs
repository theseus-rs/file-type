use crate::format::FileFormat;

pub(crate) const LINGUIST_283: FileFormat = FileFormat {
    id: 283,
    puid: "linguist/283",
    name: "Raku",
    extensions: &[
        "6pl", "6pm", "nqp", "p6", "p6l", "p6m", "pl", "pl6", "pm", "pm6", "raku", "rakumod", "t",
    ],
    media_types: &["text/x-perl"],
    internal_signatures: &[],
    related_formats: &[],
};
