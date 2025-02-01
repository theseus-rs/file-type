use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857533: FileFormat = FileFormat {
    id: 105_857_533,
    puid: "wikidata/105857533",
    name: "ScreenBuilder image",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFB, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
