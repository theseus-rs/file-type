use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1414: FileFormat = FileFormat {
    id: 1_414,
    source_type: SourceType::Pronom,
    name: "GeoGebra",
    extensions: &["geo"],
    media_types: &["application/vnd.geogebra.file"],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_415,
    }],
};
