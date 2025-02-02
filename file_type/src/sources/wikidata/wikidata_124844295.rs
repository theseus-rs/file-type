use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124844295: FileFormat = FileFormat {
    id: 124_844_295,
    source_type: SourceType::Wikidata,
    name: "CyberLink MediaShow Data",
    extensions: &["flz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
