use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130372707: FileFormat = FileFormat {
    id: 130_372_707,
    puid: "wikidata/130372707",
    name: "newLISP source code file",
    extensions: &["kif", "kif", "lsp", "lsp", "nl", "nl"],
    media_types: &[
        "application/x-newlisp",
        "application/x-newlisp",
        "application/x-newlisp",
        "text/x-newlisp",
        "text/x-newlisp",
        "text/x-newlisp",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
