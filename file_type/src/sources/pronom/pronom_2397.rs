use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2397: FileFormat = FileFormat {
    id: 2_397,
    source_type: SourceType::Pronom,
    name: "OrCAD Project File",
    extensions: &["opj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x45, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x50, 0x72, 0x6F, 0x6A, 0x65,
                    0x63, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
