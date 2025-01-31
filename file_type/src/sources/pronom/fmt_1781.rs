use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1781: FileFormat = FileFormat {
    id: 2_631,
    puid: "fmt/1781",
    name: "Pentax PEF Image File",
    extensions: &["pef"],
    media_types: &["image/dng"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x50, 0x45, 0x4E, 0x54, 0x41, 0x58]),
                    Token::WildcardCountRange(0, 2_048),
                    Token::Literal(&[0x41, 0x4F, 0x43, 0x00, 0x4D, 0x4D]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_099,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
