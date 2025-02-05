use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_852: FileFormat = FileFormat {
    id: 852,
    source_type: SourceType::Pronom,
    name: "CATIA Product Description",
    extensions: &["catproduct"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x56, 0x35, 0x5F, 0x43, 0x46, 0x56, 0x32, 0x00, 0x00]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x2E, 0x43, 0x41, 0x54, 0x50, 0x72, 0x6F, 0x64, 0x75, 0x63, 0x74,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
