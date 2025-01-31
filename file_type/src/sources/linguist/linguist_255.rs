use crate::format::FileFormat;

pub(crate) const LINGUIST_255: FileFormat = FileFormat {
    id: 255,
    puid: "linguist/255",
    name: "OCaml",
    extensions: &["eliom", "eliomi", "ml", "ml4", "mli", "mll", "mly"],
    media_types: &["text/x-ocaml"],
    internal_signatures: &[],
    related_formats: &[],
};
