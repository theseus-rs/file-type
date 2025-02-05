use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52425710: FileFormat = FileFormat {
    id: 52_425_710,
    source_type: SourceType::Wikidata,
    name: "VisiCalc Database",
    extensions: &["dif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
