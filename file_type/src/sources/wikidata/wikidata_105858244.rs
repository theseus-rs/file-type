use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858244: FileFormat = FileFormat {
    id: 105_858_244,
    puid: "wikidata/105858244",
    name: "Encrypted PhotoList",
    extensions: &["epl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x50, 0x4C, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
