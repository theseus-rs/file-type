use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858949: FileFormat = FileFormat {
    id: 105_858_949,
    puid: "wikidata/105858949",
    name: "Atari INT95a bitmap",
    extensions: &["int"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4E, 0x54, 0x39, 0x35, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
