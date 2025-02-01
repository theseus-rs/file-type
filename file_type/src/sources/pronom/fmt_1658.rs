use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1658: FileFormat = FileFormat {
    id: 2_485,
    puid: "fmt/1658",
    name: "XL-Paint MaX",
    extensions: &["max", "xlp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x4C, 0x50, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
