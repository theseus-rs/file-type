use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2865: FileFormat = FileFormat {
    id: 2_865,
    source_type: SourceType::Pronom,
    name: "Sibelius Score",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x0F, 0x53, 0x49, 0x42, 0x45, 0x4C, 0x49, 0x55, 0x53]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x00, 0x43]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_495,
    }],
};
