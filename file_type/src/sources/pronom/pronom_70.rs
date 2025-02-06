use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_70: FileFormat = FileFormat {
    id: 70,
    source_type: SourceType::Pronom,
    name: "AutoCAD dbConnect Query Set",
    extensions: &["dbq"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x00, 0x00, 0x00, 0x44, 0x42, 0x43, 0x4E, 0x43, 0x20, 0x52, 0x31, 0x35,
                    0x7C, 0x0D, 0x00, 0x00, 0x00, 0x51, 0x55, 0x45, 0x52, 0x59, 0x53, 0x45, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
