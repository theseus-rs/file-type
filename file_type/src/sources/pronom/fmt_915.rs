use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_915: FileFormat = FileFormat {
    id: 1_720,
    puid: "fmt/915",
    name: "Mapsforge Binary Map File Format",
    extensions: &["map"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6D, 0x61, 0x70, 0x73, 0x66, 0x6F, 0x72, 0x67, 0x65, 0x20, 0x62, 0x69, 0x6E,
                    0x61, 0x72, 0x79, 0x20, 0x4F, 0x53, 0x4D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
