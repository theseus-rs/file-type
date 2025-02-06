use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100243915: FileFormat = FileFormat {
    id: 100_243_915,
    source_type: SourceType::Wikidata,
    name: "Student Writing Center Journal",
    extensions: &["jn", "jnt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
