use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852782: FileFormat = FileFormat {
    id: 105_852_782,
    puid: "wikidata/105852782",
    name: "Spritemate",
    extensions: &["spm"],
    media_types: &["application/json"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x22, 0x63, 0x6F, 0x6C, 0x6F, 0x72, 0x73, 0x22, 0x3A, 0x7B, 0x22, 0x74,
                    0x22, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
