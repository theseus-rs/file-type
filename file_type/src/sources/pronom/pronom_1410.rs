use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1410: FileFormat = FileFormat {
    id: 1_410,
    source_type: SourceType::Pronom,
    name: "Windows Imaging Format",
    extensions: &["wim", "swm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x57, 0x49, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
