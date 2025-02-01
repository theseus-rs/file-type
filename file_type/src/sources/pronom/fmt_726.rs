use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_726: FileFormat = FileFormat {
    id: 1_525,
    puid: "fmt/726",
    name: "Virtual Disk Image",
    extensions: &["vdi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(64),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7F, 0x10, 0xDA, 0xBE])],
            },
        }],
    }],
    related_formats: &[],
};
