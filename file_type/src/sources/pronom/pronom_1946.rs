use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1946: FileFormat = FileFormat {
    id: 1_946,
    source_type: SourceType::Pronom,
    name: "MiniCAD",
    extensions: &["mcd"],
    media_types: &["application/vnd.vectorworks"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x08, 0x1C, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
