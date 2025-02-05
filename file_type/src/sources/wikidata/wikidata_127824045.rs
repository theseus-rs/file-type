use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127824045: FileFormat = FileFormat {
    id: 127_824_045,
    source_type: SourceType::Wikidata,
    name: "Cinema DTS Audio file format",
    extensions: &["apx", "aud", "aue"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
