use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_574: FileFormat = FileFormat {
    id: 1_362,
    puid: "fmt/574",
    name: "Digital Imaging and Communications in Medicine File Format",
    extensions: &["dcm"],
    media_types: &["application/dicom"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(128),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x49, 0x43, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
