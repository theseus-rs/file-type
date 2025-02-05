use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111317643: FileFormat = FileFormat {
    id: 111_317_643,
    source_type: SourceType::Wikidata,
    name: "Miles Sound System compressed DLS",
    extensions: &["mls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
