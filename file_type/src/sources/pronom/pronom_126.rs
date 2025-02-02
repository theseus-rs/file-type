use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_126: FileFormat = FileFormat {
    id: 126,
    source_type: SourceType::Pronom,
    name: "Lotus 1-2-3 Chart",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00,
                    0x0C, 0x7F, 0x09, 0x06,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
