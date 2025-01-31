use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850888: FileFormat = FileFormat {
    id: 105_850_888,
    puid: "wikidata/105850888",
    name: "KwikDraw drawing (v1.x)",
    extensions: &["kwk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x77, 0x69, 0x6B, 0x44, 0x72, 0x61, 0x77, 0x20, 0x76, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
