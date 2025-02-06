use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_487: FileFormat = FileFormat {
    id: 487,
    source_type: SourceType::Pronom,
    name: "Harvard Graphics Show",
    extensions: &["shw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x48, 0x4F, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
