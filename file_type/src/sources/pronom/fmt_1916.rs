use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1916: FileFormat = FileFormat {
    id: 2_774,
    puid: "fmt/1916",
    name: "Autodesk Alias Wire Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(12),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x6C, 0x69, 0x61, 0x73, 0x20, 0x50, 0x6F, 0x77, 0x65, 0x72, 0x41, 0x6E,
                    0x69, 0x6D, 0x61, 0x74, 0x6F, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x37,
                    0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
