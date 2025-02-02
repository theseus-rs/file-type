use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1052: FileFormat = FileFormat {
    id: 1_052,
    source_type: SourceType::Pronom,
    name: "Quicken Interchange Format",
    extensions: &["qif"],
    media_types: &["application/qif"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x21, 0x54, 0x79, 0x70, 0x65, 0x3A]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x5E]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
