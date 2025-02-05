use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2265: FileFormat = FileFormat {
    id: 2_265,
    source_type: SourceType::Pronom,
    name: "XLD4 (Bitmap Image)",
    extensions: &["q4"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x4A, 0x59, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
