use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1616: FileFormat = FileFormat {
    id: 1_616,
    source_type: SourceType::Pronom,
    name: "NUT Open Container Format",
    extensions: &["nut"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6E, 0x75, 0x74, 0x2F, 0x6D, 0x75, 0x6C, 0x74, 0x69, 0x6D, 0x65, 0x64, 0x69,
                    0x61, 0x20, 0x63, 0x6F, 0x6E, 0x74, 0x61, 0x69, 0x6E, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
