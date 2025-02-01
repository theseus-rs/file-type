use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1896: FileFormat = FileFormat {
    id: 2_752,
    puid: "fmt/1896",
    name: "Nokia Picture Message",
    extensions: &["npm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x50, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
