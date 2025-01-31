use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1865: FileFormat = FileFormat {
    id: 2_719,
    puid: "fmt/1865",
    name: "SWiSH Movie File",
    extensions: &["swi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x53, 0x63, 0x65, 0x6E, 0x65, 0x4F, 0x62, 0x6A,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 687,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
