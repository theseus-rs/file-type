use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1443: FileFormat = FileFormat {
    id: 2_261,
    puid: "fmt/1443",
    name: "QuarkXPress Document",
    extensions: &[],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x36, 0x00, 0x36])],
            },
        }],
    }],
    related_formats: &[],
};
