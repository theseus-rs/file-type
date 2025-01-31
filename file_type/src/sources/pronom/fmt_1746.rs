use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1746: FileFormat = FileFormat {
    id: 2_592,
    puid: "fmt/1746",
    name: "Rocky Interlace Picture",
    extensions: &["rip"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x49, 0x50, 0x31, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
