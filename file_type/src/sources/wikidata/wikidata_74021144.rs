use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_74021144: FileFormat = FileFormat {
    id: 74_021_144,
    source_type: SourceType::Wikidata,
    name: "PICS Rating System",
    extensions: &["rat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
