use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857065: FileFormat = FileFormat {
    id: 105_857_065,
    puid: "wikidata/105857065",
    name: "Vectrex game ROM",
    extensions: &["gam", "vec"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x20, 0x47, 0x43, 0x45, 0x20])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x20, 0x47, 0x43, 0x45, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};
