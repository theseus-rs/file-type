use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_687: FileFormat = FileFormat {
    id: 1_486,
    puid: "fmt/687",
    name: "Better Portable Graphics",
    extensions: &["bpg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x50, 0x47, 0xFB])],
            },
        }],
    }],
    related_formats: &[],
};
