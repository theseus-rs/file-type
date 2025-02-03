use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1707: FileFormat = FileFormat {
    id: 1_707,
    source_type: SourceType::Pronom,
    name: "Blender 3D",
    extensions: &["blend"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x42, 0x4C, 0x45, 0x4E, 0x44, 0x45, 0x52, 0x5F]),
                    Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
