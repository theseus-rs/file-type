use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1592: FileFormat = FileFormat {
    id: 1_592,
    source_type: SourceType::Pronom,
    name: "RPM Package Manager file",
    extensions: &["rpm", "src.rpm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
