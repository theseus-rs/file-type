use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856725: FileFormat = FileFormat {
    id: 105_856_725,
    puid: "wikidata/105856725",
    name: "Twist Update script",
    extensions: &["u"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
