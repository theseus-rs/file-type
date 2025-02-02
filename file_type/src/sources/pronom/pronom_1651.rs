use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1651: FileFormat = FileFormat {
    id: 1_651,
    source_type: SourceType::Pronom,
    name: "NuFile Exchange Archival Library",
    extensions: &["shk", "sdk", "bxy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0xF5, 0x46, 0xE9, 0x6C, 0xE5])],
            },
        }],
    }],
    related_formats: &[],
};
