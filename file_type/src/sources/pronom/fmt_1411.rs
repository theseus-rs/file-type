use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1411: FileFormat = FileFormat {
    id: 2_229,
    puid: "fmt/1411",
    name: "Flow Charting",
    extensions: &["pdq"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0D, 0xEF, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
