use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866068: FileFormat = FileFormat {
    id: 105_866_068,
    puid: "wikidata/105866068",
    name: "PathMinder configuration (generic)",
    extensions: &["cnf", "pm4"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4D, 0x2E, 0x43, 0x4E, 0x46])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4D, 0x2E, 0x43, 0x4E, 0x46])],
                },
            }],
        },
    ],
    related_formats: &[],
};
