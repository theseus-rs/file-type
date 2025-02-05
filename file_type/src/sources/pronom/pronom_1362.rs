use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1362: FileFormat = FileFormat {
    id: 1_362,
    source_type: SourceType::Pronom,
    name: "Digital Imaging and Communications in Medicine File Format",
    extensions: &["dcm"],
    media_types: &["application/dicom"],
    signatures: &[Signature {
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
