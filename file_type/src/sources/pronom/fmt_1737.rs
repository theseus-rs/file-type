use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1737: FileFormat = FileFormat {
    id: 2_583,
    puid: "fmt/1737",
    name: "Flow Cytometry Standard File",
    extensions: &["fcs"],
    media_types: &["application/vnd.isac.fcs"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x43, 0x53]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x20]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
