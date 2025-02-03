use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1307: FileFormat = FileFormat {
    id: 1_307,
    source_type: SourceType::Pronom,
    name: "OpenType Font File",
    extensions: &["otf"],
    media_types: &["font/otf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4F, 0x54, 0x54, 0x4F]),
                    Token::WildcardCountRange(8, 40),
                    Token::Literal(&[0x43, 0x46, 0x46, 0x20]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
