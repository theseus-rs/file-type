use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858559: FileFormat = FileFormat {
    id: 105_858_559,
    puid: "wikidata/105858559",
    name: "Multipaint image (C64 hi-res)",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x06, 0xFF, 0x00, 0x00, 0x0F, 0x28, 0x00, 0x19, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
