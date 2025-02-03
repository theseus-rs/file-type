use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2757: FileFormat = FileFormat {
    id: 2_757,
    source_type: SourceType::Pronom,
    name: "SGI Movie File",
    extensions: &["mv", "movie"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x4F, 0x56, 0x49, 0x00, 0x00, 0x00, 0x03,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
