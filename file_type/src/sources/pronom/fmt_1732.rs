use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1732: FileFormat = FileFormat {
    id: 2_577,
    puid: "fmt/1732",
    name: "Prism Paint Bitmap",
    extensions: &["pnt", "tpi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4E, 0x54, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
