use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111432414: FileFormat = FileFormat {
    id: 111_432_414,
    source_type: SourceType::Wikidata,
    name: "Lisp Program Source Code File",
    extensions: &["lsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
