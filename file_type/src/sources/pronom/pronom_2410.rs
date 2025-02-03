use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2410: FileFormat = FileFormat {
    id: 2_410,
    source_type: SourceType::Pronom,
    name: "TurboCalc Document",
    extensions: &["tcd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x0C, 0x54, 0x55, 0x52, 0x42, 0x4F, 0x43, 0x41, 0x4C, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
