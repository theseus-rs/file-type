use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1467: FileFormat = FileFormat {
    id: 2_290,
    puid: "fmt/1467",
    name: "STOS Memory Bank",
    extensions: &["mbk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x69, 0x6F, 0x6E, 0x70, 0x6F, 0x75, 0x62, 0x6E, 0x6B,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 687,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
