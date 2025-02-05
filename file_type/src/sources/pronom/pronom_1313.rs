use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1313: FileFormat = FileFormat {
    id: 1_313,
    source_type: SourceType::Pronom,
    name: "Adobe Font List",
    extensions: &["lst"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x21, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x46, 0x6F, 0x6E, 0x74, 0x4C,
                    0x69, 0x73, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
