use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1609: FileFormat = FileFormat {
    id: 2_436,
    puid: "fmt/1609",
    name: "exFAT (Extensible File Allocation Table) Disc Image",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEB, 0x76, 0x90, 0x45, 0x58, 0x46, 0x41, 0x54, 0x20, 0x20, 0x20, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_895,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
