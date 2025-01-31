use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1453: FileFormat = FileFormat {
    id: 2_275,
    puid: "fmt/1453",
    name: "Lotus 1-2-3 Worksheet",
    extensions: &["123"],
    media_types: &["application/vnd.lotus-1-2-3", "application/x-123"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x1A, 0x00, 0x05, 0x10, 0x04, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
