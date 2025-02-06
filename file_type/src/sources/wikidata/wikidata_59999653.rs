use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59999653: FileFormat = FileFormat {
    id: 59_999_653,
    source_type: SourceType::Wikidata,
    name: "ESRI Arc/View Project",
    extensions: &["apr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
