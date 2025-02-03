use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2103: FileFormat = FileFormat {
    id: 2_103,
    source_type: SourceType::Pronom,
    name: "PFS:First Choice Graph",
    extensions: &["gra"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(9),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x41, 0x42, 0x42, 0x49, 0x54, 0x47, 0x52, 0x41, 0x50, 0x48,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
