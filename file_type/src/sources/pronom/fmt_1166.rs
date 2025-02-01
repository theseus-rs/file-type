use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1166: FileFormat = FileFormat {
    id: 1_976,
    puid: "fmt/1166",
    name: "Niton Data Transfer",
    extensions: &["ndt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xBA, 0xFE, 0xD5, 0x46, 0x01, 0x00, 0x11, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
