use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1354: FileFormat = FileFormat {
    id: 2_172,
    puid: "fmt/1354",
    name: "QuickBooks Backup File",
    extensions: &["qbb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x86, 0x00, 0x00, 0x06, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
