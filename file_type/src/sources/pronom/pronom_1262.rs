use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1262: FileFormat = FileFormat {
    id: 1_262,
    source_type: SourceType::Pronom,
    name: "Microsoft Management Console Snap-in Control file",
    extensions: &["msc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x3F, 0x3E, 0x0D, 0x0A, 0x3C, 0x4D, 0x4D,
                    0x43, 0x5F, 0x43, 0x6F, 0x6E, 0x73, 0x6F, 0x6C, 0x65, 0x46, 0x69, 0x6C, 0x65,
                    0x20, 0x43, 0x6F, 0x6E, 0x73, 0x6F, 0x6C, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69,
                    0x6F, 0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 638,
    }],
};
