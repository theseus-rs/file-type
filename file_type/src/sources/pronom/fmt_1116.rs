use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1116: FileFormat = FileFormat {
    id: 1_924,
    puid: "fmt/1116",
    name: "Python Compiled File",
    extensions: &["pyc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x9E, 0x0C, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
