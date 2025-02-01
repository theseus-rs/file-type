use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851350: FileFormat = FileFormat {
    id: 105_851_350,
    puid: "wikidata/105851350",
    name: "Oric Tape image",
    extensions: &["dat", "tap"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x16, 0x16, 0x16, 0x24])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x16, 0x16, 0x16, 0x24])],
                },
            }],
        },
    ],
    related_formats: &[],
};
