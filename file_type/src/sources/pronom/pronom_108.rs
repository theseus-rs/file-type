use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_108: FileFormat = FileFormat {
    id: 108,
    source_type: SourceType::Pronom,
    name: "OS/2 Presentation Manager Metafile (MET)",
    extensions: &["met"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(2),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD3, 0xA8, 0xA8])],
            },
        }],
    }],
    related_formats: &[],
};
