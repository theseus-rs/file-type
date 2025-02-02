use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2242: FileFormat = FileFormat {
    id: 2_242,
    source_type: SourceType::Pronom,
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
