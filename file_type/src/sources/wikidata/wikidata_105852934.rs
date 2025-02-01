use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852934: FileFormat = FileFormat {
    id: 105_852_934,
    puid: "wikidata/105852934",
    name: "stz compressed data",
    extensions: &["stz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD2, 0xD9, 0x01, 0x30, 0x00, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
