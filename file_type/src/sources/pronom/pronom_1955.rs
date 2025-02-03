use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1955: FileFormat = FileFormat {
    id: 1_955,
    source_type: SourceType::Pronom,
    name: "Maxwell Render Material File",
    extensions: &["mxm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x58, 0x4D]),
                    Token::WildcardCountRange(4, 64),
                    Token::Literal(&[0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
