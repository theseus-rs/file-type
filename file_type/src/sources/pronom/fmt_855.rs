use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_855: FileFormat = FileFormat {
    id: 1_656,
    puid: "fmt/855",
    name: "Personal Ancestral File (PAF)",
    extensions: &["paf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x46, 0x00, 0x33, 0x30, 0x30, 0x00, 0x34, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_657,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_655,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
