use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1363: FileFormat = FileFormat {
    id: 2_181,
    puid: "fmt/1363",
    name: "DeluxePaint Animation File",
    extensions: &["anm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x50, 0x46, 0x20, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
