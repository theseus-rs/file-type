use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1110: FileFormat = FileFormat {
    id: 1_918,
    puid: "fmt/1110",
    name: "Python Compiled File",
    extensions: &["pyc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0xF2, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
