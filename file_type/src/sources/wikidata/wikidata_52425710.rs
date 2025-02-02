use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52425710: FileFormat = FileFormat {
    id: 52_425_710,
    source_type: SourceType::Wikidata,
    name: "VisiCalc Database",
    extensions: &["dif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
