use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2608: FileFormat = FileFormat {
    id: 2_608,
    source_type: SourceType::Pronom,
    name: "Media Descriptor Sidecar File",
    extensions: &["mds"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x45, 0x44, 0x49, 0x41, 0x20, 0x44, 0x45, 0x53, 0x43, 0x52, 0x49, 0x50,
                    0x54, 0x4F, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
