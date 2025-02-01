use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_90: FileFormat = FileFormat {
    id: 625,
    puid: "fmt/90",
    name: "PCX",
    extensions: &["pcx", "pcc"],
    media_types: &["image/vnd.zbrush.pcx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0A, 0x05, 0x01])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 687,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 624,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
