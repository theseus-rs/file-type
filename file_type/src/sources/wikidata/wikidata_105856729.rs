use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856729: FileFormat = FileFormat {
    id: 105_856_729,
    source_type: SourceType::Wikidata,
    name: "Universal Scene Description (binary)",
    extensions: &["usd", "usdc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x58, 0x52, 0x2D, 0x55, 0x53, 0x44, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
