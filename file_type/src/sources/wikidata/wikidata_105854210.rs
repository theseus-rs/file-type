use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854210: FileFormat = FileFormat {
    id: 105_854_210,
    puid: "wikidata/105854210",
    name: "NIST Sphere waveform audio",
    extensions: &["nist", "sph"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x49, 0x53, 0x54, 0x5F, 0x31, 0x41])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x49, 0x53, 0x54, 0x5F, 0x31, 0x41])],
                },
            }],
        },
    ],
    related_formats: &[],
};
