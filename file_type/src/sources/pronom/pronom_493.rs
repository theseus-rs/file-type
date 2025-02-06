use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_493: FileFormat = FileFormat {
    id: 493,
    source_type: SourceType::Pronom,
    name: "JustWrite Text Document",
    extensions: &["jw", "jwt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x46, 0x46, 0x46, 0x49, 0x49, 0x49, 0x49,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
