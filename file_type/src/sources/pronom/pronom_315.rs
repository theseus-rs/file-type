use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_315: FileFormat = FileFormat {
    id: 315,
    source_type: SourceType::Pronom,
    name: "Autodesk Animator CEL File Format",
    extensions: &["cel"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x19, 0x91]),
                    Token::NotLiteral(&[0x40, 0x01, 0xC8, 0x00, 0x00, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
