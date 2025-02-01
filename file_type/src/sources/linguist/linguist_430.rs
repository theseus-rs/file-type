use crate::format::FileFormat;

pub(crate) const LINGUIST_430: FileFormat = FileFormat {
    id: 430,
    puid: "linguist/430",
    name: "EBNF",
    extensions: &["ebnf"],
    media_types: &["text/x-ebnf"],
    internal_signatures: &[],
    related_formats: &[],
};
