use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2591: FileFormat = FileFormat {
    id: 2_591,
    source_type: SourceType::Pronom,
    name: "PixArt Bitmap",
    extensions: &["pix"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x58, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
