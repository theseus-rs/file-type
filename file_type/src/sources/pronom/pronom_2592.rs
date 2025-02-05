use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2592: FileFormat = FileFormat {
    id: 2_592,
    source_type: SourceType::Pronom,
    name: "Rocky Interlace Picture",
    extensions: &["rip"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x49, 0x50, 0x31, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
