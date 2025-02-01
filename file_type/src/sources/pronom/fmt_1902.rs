use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1902: FileFormat = FileFormat {
    id: 2_758,
    puid: "fmt/1902",
    name: "Norton Change Directory Persistent Cache File",
    extensions: &["ncd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4E, 0x43, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
