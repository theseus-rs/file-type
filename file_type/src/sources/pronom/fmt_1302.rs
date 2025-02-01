use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1302: FileFormat = FileFormat {
    id: 2_120,
    puid: "fmt/1302",
    name: "PrintMaster Gold Project",
    extensions: &["ban", "cal", "car", "let", "sig"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4C, 0x53, 0x44, 0x4F, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
