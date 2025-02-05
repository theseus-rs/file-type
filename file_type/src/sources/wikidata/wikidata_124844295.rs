use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124844295: FileFormat = FileFormat {
    id: 124_844_295,
    source_type: SourceType::Wikidata,
    name: "CyberLink MediaShow Data",
    extensions: &["flz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
