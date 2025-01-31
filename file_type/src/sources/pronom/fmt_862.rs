use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_862: FileFormat = FileFormat {
    id: 1_664,
    puid: "fmt/862",
    name: "Maya Binary File Format",
    extensions: &["mb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x38]),
                    Token::WildcardCount(12),
                    Token::Any(&[
                        &[Token::Literal(&[0x4D, 0x61, 0x79, 0x61])],
                        &[Token::Literal(&[0x4D, 0x41, 0x59, 0x41])],
                    ]),
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x38]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
