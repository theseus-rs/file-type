use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2279: FileFormat = FileFormat {
    id: 2_279,
    source_type: SourceType::Pronom,
    name: "Autocad DMP File",
    extensions: &["dmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4D, 0x61, 0x69, 0x6E, 0x5D, 0x0D, 0x0A, 0x44, 0x75, 0x6D, 0x70, 0x45,
                    0x6E, 0x74, 0x72, 0x69, 0x65, 0x73, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
