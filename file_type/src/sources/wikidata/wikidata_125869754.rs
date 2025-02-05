use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125869754: FileFormat = FileFormat {
    id: 125_869_754,
    source_type: SourceType::Wikidata,
    name: "Pro Tools Session File 5-9",
    extensions: &["ptf", "pts"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
