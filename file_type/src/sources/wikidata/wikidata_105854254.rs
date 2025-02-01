use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854254: FileFormat = FileFormat {
    id: 105_854_254,
    puid: "wikidata/105854254",
    name: "Atari800Win Plus Keyboard",
    extensions: &["a8k"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x38, 0x4B, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
