use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1936828: FileFormat = FileFormat {
    id: 1_936_828,
    puid: "wikidata/1936828",
    name: "WAD",
    extensions: &["wad", "wad"],
    media_types: &["application/octet-stream", "application/wad"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x57, 0x41, 0x44])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x57, 0x41, 0x44])],
                },
            }],
        },
    ],
    related_formats: &[],
};
