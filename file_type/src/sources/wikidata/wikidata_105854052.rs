use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854052: FileFormat = FileFormat {
    id: 105_854_052,
    puid: "wikidata/105854052",
    name: "AAX compressed data",
    extensions: &["aax"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0xFE, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
