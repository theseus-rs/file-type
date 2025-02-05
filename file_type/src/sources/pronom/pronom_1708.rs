use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1708: FileFormat = FileFormat {
    id: 1_708,
    source_type: SourceType::Pronom,
    name: "Blender 3D",
    extensions: &["blend"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x42, 0x4C, 0x45, 0x4E, 0x44, 0x45, 0x52, 0x2D]),
                    Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
