use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_733: FileFormat = FileFormat {
    id: 1_532,
    puid: "fmt/733",
    name: "FL Studio project file (FLP)",
    extensions: &["flp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4C, 0x68, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
