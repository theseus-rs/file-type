use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100244109: FileFormat = FileFormat {
    id: 100_244_109,
    source_type: SourceType::Wikidata,
    name: "Student Writing Center Newsletter",
    extensions: &["nl", "nlt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
