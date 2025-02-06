use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114050529: FileFormat = FileFormat {
    id: 114_050_529,
    source_type: SourceType::Wikidata,
    name: "Canon MIF File",
    extensions: &["mif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
