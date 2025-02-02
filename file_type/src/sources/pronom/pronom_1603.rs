use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1603: FileFormat = FileFormat {
    id: 1_603,
    source_type: SourceType::Pronom,
    name: "Encase Image File/Expert Witness Compression File",
    extensions: &["e01"],
    media_types: &["application/encase"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x56, 0x46, 0x09, 0x0D, 0x0A, 0xFF, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_255,
    }],
};
