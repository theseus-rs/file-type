use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_803: FileFormat = FileFormat {
    id: 1_603,
    puid: "fmt/803",
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
        id: 1_255,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
