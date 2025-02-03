use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2387: FileFormat = FileFormat {
    id: 2_387,
    source_type: SourceType::Pronom,
    name: "AutoDesk Indexed Point Cloud",
    extensions: &["pcg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x43, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
