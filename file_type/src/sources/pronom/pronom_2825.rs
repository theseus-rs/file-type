use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2825: FileFormat = FileFormat {
    id: 2_825,
    source_type: SourceType::Pronom,
    name: "Disklavier E-Seq Music",
    extensions: &["fil", "esq"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(7),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4F, 0x4D, 0x2D, 0x45, 0x53, 0x45, 0x51,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
