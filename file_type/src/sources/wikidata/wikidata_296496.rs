use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_296496: FileFormat = FileFormat {
    id: 296_496,
    source_type: SourceType::Wikidata,
    name: "ARC",
    extensions: &["arc", "ark", "sue"],
    media_types: &["application/x-arc"],
    internal_signatures: &[],
    related_formats: &[],
};
