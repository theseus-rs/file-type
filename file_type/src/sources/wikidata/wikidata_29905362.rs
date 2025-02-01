use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905362: FileFormat = FileFormat {
    id: 29_905_362,
    puid: "wikidata/29905362",
    name: "sK1",
    extensions: &["sk1"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x23, 0x73, 0x4B, 0x31, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
