use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858677: FileFormat = FileFormat {
    id: 105_858_677,
    puid: "wikidata/105858677",
    name: "MAG v2 bitmap",
    extensions: &["mag", "max"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x4B, 0x49, 0x30, 0x32])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x4B, 0x49, 0x30, 0x32])],
                },
            }],
        },
    ],
    related_formats: &[],
};
