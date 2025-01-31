use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1063: FileFormat = FileFormat {
    id: 1_870,
    puid: "fmt/1063",
    name: "Leaf Mosaic Raw Image",
    extensions: &["mos"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                    Token::WildcardCountRange(4, 36_864),
                    Token::Literal(&[0x50, 0x4B, 0x54, 0x53, 0x00, 0x00, 0x00, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_099,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
