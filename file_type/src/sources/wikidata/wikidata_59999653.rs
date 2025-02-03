use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59999653: FileFormat = FileFormat {
    id: 59_999_653,
    source_type: SourceType::Wikidata,
    name: "ESRI Arc/View Project",
    extensions: &["apr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
