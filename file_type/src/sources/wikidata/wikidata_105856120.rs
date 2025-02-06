use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856120: FileFormat = FileFormat {
    id: 105_856_120,
    source_type: SourceType::Wikidata,
    name: "Distribution Format Exchange Profile",
    extensions: &["dfxp"],
    media_types: &["application/ttml+xml"],
    signatures: &[],
    related_formats: &[],
};
