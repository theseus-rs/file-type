use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1177: FileFormat = FileFormat {
    id: 1_987,
    puid: "fmt/1177",
    name: "MicroStation Material Library",
    extensions: &["mat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x56, 0x5F, 0x41, 0x53, 0x2D, 0x2D, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
