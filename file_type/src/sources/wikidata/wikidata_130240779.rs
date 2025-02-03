use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130240779: FileFormat = FileFormat {
    id: 130_240_779,
    source_type: SourceType::Wikidata,
    name: "Literate Haskell source code file",
    extensions: &["lhs"],
    media_types: &["text/x-literate-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
