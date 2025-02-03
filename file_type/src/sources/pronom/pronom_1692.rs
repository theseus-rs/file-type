use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1692: FileFormat = FileFormat {
    id: 1_692,
    source_type: SourceType::Pronom,
    name: "QuadriSpace Format",
    extensions: &["qsd", "qsl", "qsm", "qst"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x9F, 0x08, 0x7C, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
