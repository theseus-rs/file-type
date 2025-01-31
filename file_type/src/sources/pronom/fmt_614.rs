use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_614: FileFormat = FileFormat {
    id: 1_410,
    puid: "fmt/614",
    name: "Windows Imaging Format",
    extensions: &["wim", "swm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x57, 0x49, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
