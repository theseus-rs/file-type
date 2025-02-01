use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858792: FileFormat = FileFormat {
    id: 105_858_792,
    puid: "wikidata/105858792",
    name: "Red Sector Demo-Maker vector-ball object",
    extensions: &["bal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x41, 0x4C, 0x4C, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
