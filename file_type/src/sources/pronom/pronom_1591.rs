use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1591: FileFormat = FileFormat {
    id: 1_591,
    source_type: SourceType::Pronom,
    name: "Unified Emulator Format",
    extensions: &["uef", "hq.uef"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x45, 0x46, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x21, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
