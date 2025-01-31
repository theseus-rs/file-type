use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852748: FileFormat = FileFormat {
    id: 105_852_748,
    puid: "wikidata/105852748",
    name: "Saxon Publisher document",
    extensions: &["sp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x50, 0x41, 0x47, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
