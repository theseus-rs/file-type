use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1547: FileFormat = FileFormat {
    id: 2_372,
    puid: "fmt/1547",
    name: "Daisy-Dot Font File",
    extensions: &["nlq"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x9B])],
            },
        }],
    }],
    related_formats: &[],
};
