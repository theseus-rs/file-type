use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854645: FileFormat = FileFormat {
    id: 105_854_645,
    puid: "wikidata/105854645",
    name: "Absolute Database file",
    extensions: &["abs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x42, 0x53, 0x30, 0x4C, 0x55, 0x54, 0x45, 0x44, 0x41, 0x54, 0x41, 0x42,
                    0x41, 0x53, 0x45, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
