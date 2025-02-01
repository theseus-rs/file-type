use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866919: FileFormat = FileFormat {
    id: 105_866_919,
    puid: "wikidata/105866919",
    name: "NEXUS format",
    extensions: &["nex", "nxs"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x4E, 0x45, 0x58, 0x55, 0x53])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x4E, 0x45, 0x58, 0x55, 0x53])],
                },
            }],
        },
    ],
    related_formats: &[],
};
