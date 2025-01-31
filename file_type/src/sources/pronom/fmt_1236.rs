use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1236: FileFormat = FileFormat {
    id: 2_054,
    puid: "fmt/1236",
    name: "EclipseCrossword Word List File",
    extensions: &["ewl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x20, 0x45, 0x63, 0x6C, 0x69, 0x70, 0x73, 0x65, 0x43, 0x72, 0x6F, 0x73,
                    0x73, 0x77, 0x6F, 0x72, 0x64, 0x20, 0x77, 0x6F, 0x72, 0x64, 0x20, 0x6C, 0x69,
                    0x73, 0x74, 0x0D, 0x0A, 0x3B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
