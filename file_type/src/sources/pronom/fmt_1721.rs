use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1721: FileFormat = FileFormat {
    id: 2_557,
    puid: "fmt/1721",
    name: "Pablo Paint Raster Image",
    extensions: &["ppp", "pa3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x42, 0x4C, 0x4F, 0x20, 0x50, 0x41, 0x43, 0x4B, 0x45, 0x44, 0x20,
                    0x50, 0x49, 0x43, 0x54, 0x55, 0x52, 0x45, 0x3A, 0x20, 0x47, 0x72, 0x6F, 0x75,
                    0x70, 0x65, 0x20, 0x43, 0x44, 0x4E, 0x44, 0x20, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 687,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
