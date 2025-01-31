use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851014: FileFormat = FileFormat {
    id: 105_851_014,
    puid: "wikidata/105851014",
    name: "Timeline schedule (v2.0)",
    extensions: &["t@0"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0xF0, 0x08, 0x00, 0x34, 0x12, 0x78, 0x56, 0x54, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
