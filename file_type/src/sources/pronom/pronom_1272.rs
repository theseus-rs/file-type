use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1272: FileFormat = FileFormat {
    id: 1_272,
    source_type: SourceType::Pronom,
    name: "Rocket Book eBook format",
    extensions: &["rb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xB0, 0x0C, 0xB0, 0x0C]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x4E, 0x55, 0x56, 0x4F]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
