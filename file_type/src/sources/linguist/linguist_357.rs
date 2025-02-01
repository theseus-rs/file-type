use crate::format::FileFormat;

pub(crate) const LINGUIST_357: FileFormat = FileFormat {
    id: 357,
    puid: "linguist/357",
    name: "Standard ML",
    extensions: &["fun", "ml", "sig", "sml"],
    media_types: &["text/x-ocaml"],
    internal_signatures: &[],
    related_formats: &[],
};
