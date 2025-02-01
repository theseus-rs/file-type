use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1107: FileFormat = FileFormat {
    id: 1_915,
    puid: "fmt/1107",
    name: "Python Compiled File",
    extensions: &["pyc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2A, 0xEB, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
