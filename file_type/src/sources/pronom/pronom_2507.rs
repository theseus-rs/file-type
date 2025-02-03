use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2507: FileFormat = FileFormat {
    id: 2_507,
    source_type: SourceType::Pronom,
    name: "Z Compressed Data",
    extensions: &["z"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0x9D, 0x90])],
            },
        }],
    }],
    related_formats: &[],
};
