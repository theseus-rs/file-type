use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_8041715: FileFormat = FileFormat {
    id: 8_041_715,
    source_type: SourceType::Wikidata,
    name: "XCOFF",
    extensions: &["o"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
