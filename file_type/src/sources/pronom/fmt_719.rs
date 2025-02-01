use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_719: FileFormat = FileFormat {
    id: 1_518,
    puid: "fmt/719",
    name: "MultiTracker Module",
    extensions: &["mtm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x54, 0x4D, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
