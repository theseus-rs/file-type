use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856967: FileFormat = FileFormat {
    id: 105_856_967,
    source_type: SourceType::Wikidata,
    name: "jGRASP Project",
    extensions: &["gpj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
