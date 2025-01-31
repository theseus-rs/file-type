use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_976: FileFormat = FileFormat {
    id: 1_781,
    puid: "fmt/976",
    name: "MagicaVoxel Vox format",
    extensions: &["vox"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x4F, 0x58, 0x20, 0x96, 0x00, 0x00, 0x00, 0x4D, 0x41, 0x49, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
