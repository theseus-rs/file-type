use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2199: FileFormat = FileFormat {
    id: 2_199,
    source_type: SourceType::Pronom,
    name: "VariCAD Drawing",
    extensions: &["dwb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(1),
            regex: Regex {
                tokens: &[Token::Literal(&[0x87, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
