use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_618: FileFormat = FileFormat {
    id: 1_414,
    puid: "fmt/618",
    name: "GeoGebra",
    extensions: &["geo"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xAC, 0xED, 0x00, 0x05, 0x73, 0x72, 0x00, 0x1B, 0x67, 0x65, 0x6F, 0x67, 0x65,
                    0x62, 0x72, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_415,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
