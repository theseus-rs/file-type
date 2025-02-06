use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_176: FileFormat = FileFormat {
    id: 176,
    source_type: SourceType::Pronom,
    name: "Quattro Pro Spreadsheet for DOS",
    extensions: &["wq1", "wkq"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x20, 0x51])],
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
