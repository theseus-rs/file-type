use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2579: FileFormat = FileFormat {
    id: 2_579,
    source_type: SourceType::Pronom,
    name: "Portfolio Graphics Compressed File",
    extensions: &["pgc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x47, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
