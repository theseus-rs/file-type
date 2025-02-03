use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2576: FileFormat = FileFormat {
    id: 2_576,
    source_type: SourceType::Pronom,
    name: "PowerGraphics Image File",
    extensions: &["pgr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(8),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6F, 0x77, 0x65, 0x72, 0x47, 0x46, 0x58,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
