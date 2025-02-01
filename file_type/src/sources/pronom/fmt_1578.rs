use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1578: FileFormat = FileFormat {
    id: 2_403,
    puid: "fmt/1578",
    name: "Spectrum 512 Extended",
    extensions: &["spx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x58, 0x02])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_402,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
