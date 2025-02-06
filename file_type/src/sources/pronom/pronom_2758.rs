use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2758: FileFormat = FileFormat {
    id: 2_758,
    source_type: SourceType::Pronom,
    name: "Norton Change Directory Persistent Cache File",
    extensions: &["ncd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4E, 0x43, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
