use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_394: FileFormat = FileFormat {
    id: 1_142,
    puid: "fmt/394",
    name: "DS_Store File (MAC)",
    extensions: &["ds_store"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x42, 0x75, 0x64, 0x31, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
