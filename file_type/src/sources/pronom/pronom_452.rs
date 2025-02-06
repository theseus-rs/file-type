use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_452: FileFormat = FileFormat {
    id: 452,
    source_type: SourceType::Pronom,
    name: "X-Windows Bitmap Image",
    extensions: &["xbm"],
    media_types: &["image/x-xbitmap"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x23, 0x64, 0x65, 0x66, 0x69, 0x6E, 0x65, 0x20]),
                    Token::WildcardCountRange(1, 30),
                    Token::Literal(&[0x5F, 0x77, 0x69, 0x64, 0x74, 0x68, 0x20]),
                    Token::WildcardCountRange(1, 16),
                    Token::Literal(&[0x23, 0x64, 0x65, 0x66, 0x69, 0x6E, 0x65, 0x20]),
                    Token::WildcardCountRange(1, 30),
                    Token::Literal(&[0x5F, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x20]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x75, 0x6E, 0x73, 0x69, 0x67,
                        0x6E, 0x65, 0x64, 0x20, 0x73, 0x68, 0x6F, 0x72, 0x74, 0x20,
                    ]),
                    Token::WildcardCountRange(1, 30),
                    Token::Literal(&[
                        0x5F, 0x62, 0x69, 0x74, 0x73, 0x5B, 0x5D, 0x20, 0x3D, 0x20, 0x7B,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
