use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1593: FileFormat = FileFormat {
    id: 1_593,
    source_type: SourceType::Pronom,
    name: "RPM Package Manager file",
    extensions: &["rpm", "src.rpm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
