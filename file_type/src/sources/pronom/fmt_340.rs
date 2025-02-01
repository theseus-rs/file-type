use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_340: FileFormat = FileFormat {
    id: 1_085,
    puid: "fmt/340",
    name: "Lotus WordPro Document",
    extensions: &["lwp"],
    media_types: &["application/lwp", "application/vnd.lotus-wordpro"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x6F, 0x72, 0x64, 0x50, 0x72, 0x6F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x4C, 0x57, 0x50, 0x37, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
                    0x00, 0x00, 0x00, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 504,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
