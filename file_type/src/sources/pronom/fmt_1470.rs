use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1470: FileFormat = FileFormat {
    id: 2_293,
    puid: "fmt/1470",
    name: "MIG Graphics File",
    extensions: &["mig"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x58, 0x4D, 0x49, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
