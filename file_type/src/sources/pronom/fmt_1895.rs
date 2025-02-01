use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1895: FileFormat = FileFormat {
    id: 2_751,
    puid: "fmt/1895",
    name: "RagTime Document File",
    extensions: &["rtd", "rtt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x23, 0x2B, 0x44, 0xA4, 0x43, 0x4D, 0xA5, 0x48, 0x64, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
