use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1899: FileFormat = FileFormat {
    id: 2_755,
    puid: "fmt/1899",
    name: "RIS Citation",
    extensions: &["ris"],
    media_types: &["application/x-research-info-systems"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x59, 0x20])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x52, 0x20, 0x20, 0x2D])],
                },
            },
        ],
    }],
    related_formats: &[],
};
