use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1051: FileFormat = FileFormat {
    id: 1_856,
    puid: "fmt/1051",
    name: "Windows Journal Format",
    extensions: &["jnt", "jtp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4E, 0x42, 0x2A, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x50, 0x01, 0x00, 0x10, 0x02, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
