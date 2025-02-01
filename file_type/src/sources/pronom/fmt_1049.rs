use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1049: FileFormat = FileFormat {
    id: 1_854,
    puid: "fmt/1049",
    name: "QuickDraw 3D Metafile (ASCII)",
    extensions: &["3dmf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x44, 0x4D, 0x65, 0x74, 0x61, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x28,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
