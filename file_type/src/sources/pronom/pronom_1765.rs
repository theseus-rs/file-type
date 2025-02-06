use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1765: FileFormat = FileFormat {
    id: 1_765,
    source_type: SourceType::Pronom,
    name: "DOS Sound and Music Interface Advanced Module Format",
    extensions: &["amf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x41, 0x4D, 0x46]),
                    Token::Range(&[0x0A], &[0x0E]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
