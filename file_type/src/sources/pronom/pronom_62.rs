use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_62: FileFormat = FileFormat {
    id: 62,
    source_type: SourceType::Pronom,
    name: "CorelDraw Compressed Drawing",
    extensions: &["cdx", "cjw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x44, 0x52, 0x43, 0x4F, 0x4D, 0x50, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
