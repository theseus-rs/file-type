use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2391: FileFormat = FileFormat {
    id: 2_391,
    source_type: SourceType::Pronom,
    name: "ColdFusion Markup Language",
    extensions: &["cfm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x63, 0x66])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x63, 0x66]),
                        Token::WildcardCountRange(5, 100),
                        Token::Literal(&[0x3E]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[],
};
