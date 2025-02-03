use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1124114: FileFormat = FileFormat {
    id: 1_124_114,
    source_type: SourceType::Wikidata,
    name: "LandXML",
    extensions: &["ddf", "dem", "xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
