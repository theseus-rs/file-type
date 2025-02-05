use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2119: FileFormat = FileFormat {
    id: 2_119,
    source_type: SourceType::Pronom,
    name: "The Print Shop Project",
    extensions: &["psproj"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(12),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x44, 0x35, 0x52, 0x44, 0x4F, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
