use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1792: FileFormat = FileFormat {
    id: 1_792,
    source_type: SourceType::Pronom,
    name: "Microsoft OneNote Package File",
    extensions: &["onepkg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x53, 0x43, 0x46]),
                    Token::WildcardCount(20),
                    Token::Literal(&[0x03, 0x01]),
                    Token::WildcardCountRange(1, 2_048),
                    Token::Literal(&[0x6F, 0x6E, 0x65, 0x74, 0x6F, 0x63]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 801,
    }],
};
