use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_954199: FileFormat = FileFormat {
    id: 954_199,
    puid: "wikidata/954199",
    name: "MHTML",
    extensions: &["mht", "mhtml"],
    media_types: &["message/rfc822", "message/rfc822"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x72, 0x6F, 0x6D, 0x3A, 0x20])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x72, 0x6F, 0x6D, 0x3A, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};
