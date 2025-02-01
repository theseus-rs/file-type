use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_202: FileFormat = FileFormat {
    id: 279,
    puid: "x-fmt/202",
    name: "Corel Wavelet Compressed Bitmap",
    extensions: &["wi", "wvl"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x52, 0x57, 0x41, 0x56])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x57, 0x49, 0x04]),
                        Token::Range(&[0x01], &[0x06]),
                        Token::WildcardCount(3),
                        Token::Range(&[0x40], &[0x46]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
