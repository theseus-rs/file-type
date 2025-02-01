use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_249: FileFormat = FileFormat {
    id: 365,
    puid: "x-fmt/249",
    name: "Microsoft Outlook Personal Folders (Unicode)",
    extensions: &["pst"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x21, 0x42, 0x44, 0x4E]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x53, 0x4D, 0x17, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
