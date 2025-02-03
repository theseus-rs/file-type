use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_153: FileFormat = FileFormat {
    id: 153,
    source_type: SourceType::Pronom,
    name: "AutoCAD Slide",
    extensions: &["sld"],
    media_types: &["application/sld", "application/x-sld", "image/x-sld"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x53, 0x6C, 0x69, 0x64, 0x65,
                    0x0D, 0x0A, 0x1A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
