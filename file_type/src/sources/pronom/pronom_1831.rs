use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1831: FileFormat = FileFormat {
    id: 1_831,
    source_type: SourceType::Pronom,
    name: "Statistical Analysis System Catalog (Unix)",
    extensions: &["sas7bcat", "sc7"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0xC2, 0xEA, 0x81, 0x63, 0xB3, 0x14, 0x11, 0xCF, 0xBD, 0x92, 0x08, 0x00,
                        0x09, 0xC7, 0x31, 0x8C, 0x18, 0x1F, 0x10, 0x11,
                    ]),
                    Token::WildcardCount(7),
                    Token::Literal(&[0x31]),
                    Token::WildcardCount(116),
                    Token::Literal(&[0x43, 0x41, 0x54, 0x41, 0x4C, 0x4F, 0x47]),
                    Token::WildcardCountRange(53, 61),
                    Token::Literal(&[0x39, 0x2E, 0x30, 0x32]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_829,
    }],
};
