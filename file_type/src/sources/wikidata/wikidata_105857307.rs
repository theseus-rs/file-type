use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857307: FileFormat = FileFormat {
    id: 105_857_307,
    puid: "wikidata/105857307",
    name: "Hi-MD Minidisc ATRAC3+ audio data container",
    extensions: &["hma"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x54, 0x58, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
