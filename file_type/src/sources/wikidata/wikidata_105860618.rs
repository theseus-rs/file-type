use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860618: FileFormat = FileFormat {
    id: 105_860_618,
    puid: "wikidata/105860618",
    name: "RKP game package (v1.x)",
    extensions: &["rkp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x4B, 0x00, 0x00, 0x56, 0x31, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
