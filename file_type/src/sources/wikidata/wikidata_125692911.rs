use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125692911: FileFormat = FileFormat {
    id: 125_692_911,
    source_type: SourceType::Wikidata,
    name: "StarImpress 4.0/5.0",
    extensions: &["sdp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
