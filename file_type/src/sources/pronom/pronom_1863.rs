use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1863: FileFormat = FileFormat {
    id: 1_863,
    source_type: SourceType::Pronom,
    name: "SNAP Processed Data File",
    extensions: &["snpdf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4E, 0x41, 0x50, 0x20, 0x50, 0x72, 0x6F, 0x63, 0x65, 0x73, 0x73, 0x65,
                    0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
