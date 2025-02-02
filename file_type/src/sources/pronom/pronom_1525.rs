use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1525: FileFormat = FileFormat {
    id: 1_525,
    source_type: SourceType::Pronom,
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
