use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851314: FileFormat = FileFormat {
    id: 105_851_314,
    source_type: SourceType::Wikidata,
    name: "THX Tracker instrument",
    extensions: &["ins"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x48, 0x58, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
