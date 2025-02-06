use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1423: FileFormat = FileFormat {
    id: 1_423,
    source_type: SourceType::Pronom,
    name: "Stuffit Archive File",
    extensions: &["sit"],
    media_types: &["application/x-stuffit"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x74, 0x75, 0x66, 0x66, 0x49, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
