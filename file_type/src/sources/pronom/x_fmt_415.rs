use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_415: FileFormat = FileFormat {
    id: 802,
    puid: "x-fmt/415",
    name: "Java Class File",
    extensions: &["class"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_900,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
