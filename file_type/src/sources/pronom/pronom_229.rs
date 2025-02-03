use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_229: FileFormat = FileFormat {
    id: 229,
    source_type: SourceType::Pronom,
    name: "Adobe FrameMaker Interchange Format",
    extensions: &["mif"],
    media_types: &["application/vnd.mif"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x3C, 0x4D, 0x49, 0x46, 0x46, 0x69, 0x6C, 0x65, 0x20]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x3E, 0x20, 0x23]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
