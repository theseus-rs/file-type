use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128221992: FileFormat = FileFormat {
    id: 128_221_992,
    source_type: SourceType::Wikidata,
    name: "Zimbu file",
    extensions: &["zu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
