use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125692911: FileFormat = FileFormat {
    id: 125_692_911,
    source_type: SourceType::Wikidata,
    name: "StarImpress 4.0/5.0",
    extensions: &["sdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
