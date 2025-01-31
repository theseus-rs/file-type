use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_439: FileFormat = FileFormat {
    id: 851,
    puid: "x-fmt/439",
    name: "CATIA Model (Part Description)",
    extensions: &["catpart"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x56, 0x35, 0x5F, 0x43, 0x46, 0x56, 0x32, 0x00, 0x00]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x2E, 0x43, 0x41, 0x54, 0x50, 0x61, 0x72, 0x74]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
