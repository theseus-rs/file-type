use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_152: FileFormat = FileFormat {
    id: 152,
    source_type: SourceType::Pronom,
    name: "AutoCAD Slide Library",
    extensions: &["slb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x53, 0x6C, 0x69, 0x64, 0x65,
                    0x20, 0x4C, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
