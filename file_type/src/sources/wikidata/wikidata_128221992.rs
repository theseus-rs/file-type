use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128221992: FileFormat = FileFormat {
    id: 128_221_992,
    source_type: SourceType::Wikidata,
    name: "Zimbu file",
    extensions: &["zu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
