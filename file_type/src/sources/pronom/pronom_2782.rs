use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2782: FileFormat = FileFormat {
    id: 2_782,
    source_type: SourceType::Pronom,
    name: "LiveCode Stack",
    extensions: &["rev", "livecode"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x45, 0x56, 0x4F, 0x38, 0x31, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
