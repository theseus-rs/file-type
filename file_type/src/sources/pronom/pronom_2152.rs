use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2152: FileFormat = FileFormat {
    id: 2_152,
    source_type: SourceType::Pronom,
    name: "Sony PictureGear Studio PrintStudio",
    extensions: &["lmu", "lmd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x4D, 0x55, 0x44, 0x00, 0x00, 0x01, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
