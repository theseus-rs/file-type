use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2759: FileFormat = FileFormat {
    id: 2_759,
    source_type: SourceType::Pronom,
    name: "Garmin Vehicle Images File",
    extensions: &["srf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x41, 0x52, 0x4D, 0x49, 0x4E, 0x20, 0x42, 0x49, 0x54, 0x4D, 0x41, 0x50,
                    0x20, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
