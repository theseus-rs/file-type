use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856931: FileFormat = FileFormat {
    id: 105_856_931,
    puid: "wikidata/105856931",
    name: "GoodWay Flight Planner flight plan",
    extensions: &["gwp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x57, 0x20, 0x50, 0x72, 0x65, 0x66, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                    0x6E, 0x3B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
