use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1906: FileFormat = FileFormat {
    id: 1_906,
    source_type: SourceType::Pronom,
    name: "TCR eBook",
    extensions: &["tcr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x21, 0x38, 0x2D, 0x42, 0x69, 0x74, 0x21, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
