use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854311: FileFormat = FileFormat {
    id: 105_854_311,
    puid: "wikidata/105854311",
    name: "Scifer Archiver compressed Binary Archive",
    extensions: &["ba", "sen"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC, 0xEA, 0xFA, 0xCE])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC, 0xEA, 0xFA, 0xCE])],
                },
            }],
        },
    ],
    related_formats: &[],
};
