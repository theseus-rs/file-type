use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856123: FileFormat = FileFormat {
    id: 105_856_123,
    puid: "wikidata/105856123",
    name: "Weresc CADE drawing",
    extensions: &["dtc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0C, 0x1E, 0x43, 0x41, 0x44, 0x45, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x01,
                    0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
