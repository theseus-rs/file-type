use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1076: FileFormat = FileFormat {
    id: 1_076,
    source_type: SourceType::Pronom,
    name: "Autorun Configuration File",
    extensions: &["inf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x5B, 0x61, 0x75, 0x74, 0x6F, 0x72, 0x75, 0x6E, 0x5D]),
                    Token::WildcardCountRange(2, 4),
                    Token::Any(&[
                        &[Token::Literal(&[0x4F, 0x50, 0x45, 0x4E])],
                        &[Token::Literal(&[0x6F, 0x70, 0x65, 0x6E])],
                        &[Token::Literal(&[0x49, 0x43, 0x4F, 0x4E])],
                        &[Token::Literal(&[0x69, 0x63, 0x6F, 0x6E])],
                    ]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[0x3D]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
