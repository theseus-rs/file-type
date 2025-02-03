use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2728: FileFormat = FileFormat {
    id: 2_728,
    source_type: SourceType::Pronom,
    name: "Esko ArtPro File",
    extensions: &["ap"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x72, 0x74, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
