use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_498: FileFormat = FileFormat {
    id: 1_285,
    puid: "fmt/498",
    name: "ActiveX License Package file",
    extensions: &["lpk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x50, 0x4B, 0x20, 0x4C, 0x69, 0x63, 0x65, 0x6E, 0x73, 0x65, 0x20, 0x50,
                    0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x0D, 0x0A, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
