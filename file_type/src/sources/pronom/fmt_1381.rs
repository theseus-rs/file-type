use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1381: FileFormat = FileFormat {
    id: 2_199,
    puid: "fmt/1381",
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
