use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1388: FileFormat = FileFormat {
    id: 2_206,
    puid: "fmt/1388",
    name: "Muvee Reveal Project File",
    extensions: &["rvl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(40),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x6D, 0x75, 0x76, 0x65, 0x65, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74,
                    0x3E, 0x3C, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 638,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
