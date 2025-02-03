use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2418: FileFormat = FileFormat {
    id: 2_418,
    source_type: SourceType::Pronom,
    name: "ESRI ArcInfo Coverage Annotation File",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x27, 0x0A, 0xFF, 0xFF, 0xFF, 0xBD,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
