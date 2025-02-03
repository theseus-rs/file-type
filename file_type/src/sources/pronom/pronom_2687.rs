use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2687: FileFormat = FileFormat {
    id: 2_687,
    source_type: SourceType::Pronom,
    name: "Archiver Format",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
