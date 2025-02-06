use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2704: FileFormat = FileFormat {
    id: 2_704,
    source_type: SourceType::Pronom,
    name: "DAV Video Format",
    extensions: &["dav"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x48, 0x41, 0x56])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x68, 0x61, 0x76])],
                },
            },
        ],
    }],
    related_formats: &[],
};
