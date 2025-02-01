use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858893: FileFormat = FileFormat {
    id: 105_858_893,
    puid: "wikidata/105858893",
    name: "Multipaint image (C64 multicolor)",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x06, 0x00, 0x00, 0x0A, 0x0F, 0x28, 0x00, 0x19, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
