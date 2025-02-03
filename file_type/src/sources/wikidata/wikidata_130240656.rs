use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130240656: FileFormat = FileFormat {
    id: 130_240_656,
    source_type: SourceType::Wikidata,
    name: "Literate Cryptol source code file",
    extensions: &["lcry"],
    media_types: &["text/x-literate-cryptol"],
    internal_signatures: &[],
    related_formats: &[],
};
