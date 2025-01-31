use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856995: FileFormat = FileFormat {
    id: 105_856_995,
    puid: "wikidata/105856995",
    name: "GUEmap document",
    extensions: &["gmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC7, 0x55, 0xC5, 0x6D, 0xE1, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
