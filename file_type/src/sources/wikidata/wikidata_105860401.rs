use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860401: FileFormat = FileFormat {
    id: 105_860_401,
    puid: "wikidata/105860401",
    name: "DevExpress Report layout (v1)",
    extensions: &["repx"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x2F, 0x2F, 0x20, 0x3C, 0x58, 0x52, 0x54, 0x79, 0x70, 0x65, 0x49, 0x6E,
                    0x66, 0x6F, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
