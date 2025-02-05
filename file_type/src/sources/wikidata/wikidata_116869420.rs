use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116869420: FileFormat = FileFormat {
    id: 116_869_420,
    source_type: SourceType::Wikidata,
    name: "Broderbund Print Meta Object",
    extensions: &["pmo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
