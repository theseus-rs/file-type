use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1295: FileFormat = FileFormat {
    id: 1_295,
    source_type: SourceType::Pronom,
    name: "Quarter Inch Cartridge Host Interchange Format",
    extensions: &["qic"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x56, 0x54, 0x42, 0x4C]),
                    Token::WildcardCount(54),
                    Token::Any(&[
                        &[Token::Literal(&[0x41, 0x53])],
                        &[Token::Literal(&[0x71, 0x00])],
                    ]),
                    Token::WildcardCount(66),
                    Token::Literal(&[0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
