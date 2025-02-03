use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2869: FileFormat = FileFormat {
    id: 2_869,
    source_type: SourceType::Pronom,
    name: "WinFax Fax Image",
    extensions: &["fxr", "fxm", "fxs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x0B, 0x23]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0xC0, 0x06]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
