use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853960: FileFormat = FileFormat {
    id: 105_853_960,
    puid: "wikidata/105853960",
    name: "mkwACT lossless compressed audio (v4)",
    extensions: &["mkw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x6B, 0x77, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
