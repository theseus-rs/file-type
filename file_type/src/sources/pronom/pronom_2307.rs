use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2307: FileFormat = FileFormat {
    id: 2_307,
    source_type: SourceType::Pronom,
    name: "JPEG XL Codestream",
    extensions: &["jxl"],
    media_types: &["image/jxl"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
