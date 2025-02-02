use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1325: FileFormat = FileFormat {
    id: 1_325,
    source_type: SourceType::Pronom,
    name: "Adobe FrameMaker Document",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4D, 0x61, 0x6B, 0x65, 0x72, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x37, 0x2E,
                    0x30, 0x48, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 456,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_324,
        },
    ],
};
