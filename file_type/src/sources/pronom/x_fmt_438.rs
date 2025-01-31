use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_438: FileFormat = FileFormat {
    id: 849,
    puid: "x-fmt/438",
    name: "CATIA Material Description",
    extensions: &["catmaterial"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x56, 0x35, 0x5F, 0x43, 0x46, 0x56, 0x32, 0x00, 0x00]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x2E, 0x43, 0x41, 0x54, 0x4D, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6C,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
