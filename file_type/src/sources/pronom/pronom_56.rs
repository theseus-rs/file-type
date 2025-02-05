use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_56: FileFormat = FileFormat {
    id: 56,
    source_type: SourceType::Pronom,
    name: "AutoCAD Batch Plot File",
    extensions: &["bp2", "bpl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x45, 0x78, 0x74, 0x65, 0x6E,
                    0x64, 0x65, 0x64, 0x20, 0x42, 0x61, 0x74, 0x63, 0x68, 0x20, 0x50, 0x6C, 0x6F,
                    0x74, 0x20, 0x4C, 0x69, 0x73, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
