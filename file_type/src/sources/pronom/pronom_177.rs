use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_177: FileFormat = FileFormat {
    id: 177,
    source_type: SourceType::Pronom,
    name: "Quattro Pro Spreadsheet for DOS",
    extensions: &["wq2", "wkq"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x21, 0x51])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x00, 0x00, 0x00])],
                },
            },
        ],
    }],
    related_formats: &[],
};
