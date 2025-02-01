use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1673: FileFormat = FileFormat {
    id: 2_509,
    puid: "fmt/1673",
    name: "ZBrush MatCap",
    extensions: &["zmt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x42, 0x72, 0x75, 0x73, 0x68, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
