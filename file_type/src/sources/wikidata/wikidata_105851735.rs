use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851735: FileFormat = FileFormat {
    id: 105_851_735,
    puid: "wikidata/105851735",
    name: "Beyond Words Composer Style",
    extensions: &["sty"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x42, 0x57, 0x44, 0x42, 0x5D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
