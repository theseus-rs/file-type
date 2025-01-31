use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_87: FileFormat = FileFormat {
    id: 622,
    puid: "fmt/87",
    name: "PCX",
    extensions: &["pcx", "pcc"],
    media_types: &["image/vnd.zbrush.pcx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0A, 0x02, 0x01])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 623,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 621,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
