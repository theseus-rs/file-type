use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2323: FileFormat = FileFormat {
    id: 2_323,
    source_type: SourceType::Pronom,
    name: "Adobe Acrobat Forms Data Format",
    extensions: &["fdf"],
    media_types: &["application/vnd.fdf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x46, 0x44, 0x46, 0x2D, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
