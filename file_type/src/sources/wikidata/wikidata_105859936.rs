use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859936: FileFormat = FileFormat {
    id: 105_859_936,
    puid: "wikidata/105859936",
    name: "Electronic Arts ASF video (generic)",
    extensions: &["asf", "str"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x43, 0x48, 0x6C])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x43, 0x48, 0x6C])],
                },
            }],
        },
    ],
    related_formats: &[],
};
