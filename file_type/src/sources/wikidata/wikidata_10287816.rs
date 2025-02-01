use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_10287816: FileFormat = FileFormat {
    id: 10_287_816,
    puid: "wikidata/10287816",
    name: "GZIP",
    extensions: &["gz", "gzip"],
    media_types: &["application/gzip", "application/gzip"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1F, 0x8B, 0x08])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1F, 0x8B, 0x08])],
                },
            }],
        },
    ],
    related_formats: &[],
};
