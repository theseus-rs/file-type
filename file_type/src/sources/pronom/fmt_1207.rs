use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1207: FileFormat = FileFormat {
    id: 2_017,
    puid: "fmt/1207",
    name: "Sony SFK File",
    extensions: &["sfk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x46, 0x50, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
