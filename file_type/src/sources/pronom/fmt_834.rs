use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_834: FileFormat = FileFormat {
    id: 1_635,
    puid: "fmt/834",
    name: "Quattro Pro Spreadsheet for Windows",
    extensions: &["wb1"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x01, 0x10])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_636,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
