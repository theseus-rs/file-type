use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_280: FileFormat = FileFormat {
    id: 1_020,
    puid: "fmt/280",
    name: "LaTeX (Master document)",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5C, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x63, 0x6C, 0x61, 0x73,
                    0x73,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_021,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
