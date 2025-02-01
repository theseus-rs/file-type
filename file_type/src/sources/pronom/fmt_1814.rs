use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1814: FileFormat = FileFormat {
    id: 2_665,
    puid: "fmt/1814",
    name: "Adobe Color Book for Windows",
    extensions: &["acb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x38, 0x42, 0x43, 0x42, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
