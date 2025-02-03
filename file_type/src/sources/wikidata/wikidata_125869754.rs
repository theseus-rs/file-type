use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125869754: FileFormat = FileFormat {
    id: 125_869_754,
    source_type: SourceType::Wikidata,
    name: "Pro Tools Session File 5-9",
    extensions: &["ptf", "pts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
