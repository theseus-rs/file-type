use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2672: FileFormat = FileFormat {
    id: 2_672,
    source_type: SourceType::Pronom,
    name: "MacCaption Project",
    extensions: &["cca"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x61, 0x63, 0x43, 0x61, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x5F, 0x43, 0x43,
                    0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
