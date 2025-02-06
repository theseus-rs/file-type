use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130372707: FileFormat = FileFormat {
    id: 130_372_707,
    source_type: SourceType::Wikidata,
    name: "newLISP source code file",
    extensions: &["kif", "lsp", "nl"],
    media_types: &["application/x-newlisp", "text/x-newlisp"],
    signatures: &[],
    related_formats: &[],
};
