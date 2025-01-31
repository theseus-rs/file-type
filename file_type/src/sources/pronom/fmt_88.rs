use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_88: FileFormat = FileFormat {
    id: 623,
    puid: "fmt/88",
    name: "PCX",
    extensions: &["pcx", "pcc"],
    media_types: &["image/vnd.zbrush.pcx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0A, 0x03, 0x01])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 624,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 622,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
