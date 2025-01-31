use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865831: FileFormat = FileFormat {
    id: 105_865_831,
    puid: "wikidata/105865831",
    name: "FoxPro compressed dist. archive (additional parts)",
    extensions: &["pa1", "pa2"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x50, 0x41, 0x43, 0x46, 0x50, 0x50, 0x46,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x50, 0x41, 0x43, 0x46, 0x50, 0x50, 0x46,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
