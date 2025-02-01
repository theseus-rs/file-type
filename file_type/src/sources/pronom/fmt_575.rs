use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_575: FileFormat = FileFormat {
    id: 1_363,
    puid: "fmt/575",
    name: "GraphPad Prism",
    extensions: &["pzm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x46, 0x46, 0x47, 0x52, 0x41, 0x46,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
