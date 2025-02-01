use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1996: FileFormat = FileFormat {
    id: 2_870,
    puid: "fmt/1996",
    name: "SPIR-V",
    extensions: &["spirv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x03, 0x02, 0x23, 0x07, 0x00, 0x00, 0x01, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_260,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
