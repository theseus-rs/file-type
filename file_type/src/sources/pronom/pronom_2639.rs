use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2639: FileFormat = FileFormat {
    id: 2_639,
    source_type: SourceType::Pronom,
    name: "GX2 Graphics File",
    extensions: &["gx2", "ega"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x58, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
