use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1424: FileFormat = FileFormat {
    id: 2_242,
    puid: "fmt/1424",
    name: "WordPerfect Encrypted Document",
    extensions: &["wp"],
    media_types: &["application/vnd.wordperfect"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0xFF, 0x61, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
