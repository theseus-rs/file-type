use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2688: FileFormat = FileFormat {
    id: 2_688,
    source_type: SourceType::Pronom,
    name: "Brio Query File",
    extensions: &["bqy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x42, 0x52, 0x49, 0x46, 0x20, 0x42, 0x49, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
