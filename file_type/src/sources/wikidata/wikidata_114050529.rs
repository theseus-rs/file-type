use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114050529: FileFormat = FileFormat {
    id: 114_050_529,
    source_type: SourceType::Wikidata,
    name: "Canon MIF File",
    extensions: &["mif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
