use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130240656: FileFormat = FileFormat {
    id: 130_240_656,
    source_type: SourceType::Wikidata,
    name: "Literate Cryptol source code file",
    extensions: &["lcry"],
    media_types: &["text/x-literate-cryptol"],
    signatures: &[],
    related_formats: &[],
};
