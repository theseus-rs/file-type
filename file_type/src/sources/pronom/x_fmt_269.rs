use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_269: FileFormat = FileFormat {
    id: 389,
    puid: "x-fmt/269",
    name: "ZOO Compressed Archive",
    extensions: &["zoo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(20),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDC, 0xA7, 0xC4, 0xFD])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFC, 0x83])],
                },
            },
        ],
    }],
    related_formats: &[],
};
