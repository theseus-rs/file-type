use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1659: FileFormat = FileFormat {
    id: 2_486,
    puid: "fmt/1659",
    name: "XL-Paint",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x4C, 0x50, 0x42])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 687,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
