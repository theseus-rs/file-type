use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2288: FileFormat = FileFormat {
    id: 2_288,
    source_type: SourceType::Pronom,
    name: "OrCAD Layout File",
    extensions: &["max"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x2C, 0x00, 0x4F, 0x72, 0x43, 0x41, 0x44, 0x20, 0x49, 0x4E, 0x43,
                    0x2E, 0x2C, 0x20, 0x4C, 0x61, 0x79, 0x6F, 0x75, 0x74, 0x20, 0x44, 0x61, 0x74,
                    0x61, 0x62, 0x61, 0x73, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
