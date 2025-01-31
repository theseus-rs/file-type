use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1552: FileFormat = FileFormat {
    id: 2_377,
    puid: "fmt/1552",
    name: "Surprise! Adlib Tracker v2.0",
    extensions: &["sa2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x41, 0x64, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
