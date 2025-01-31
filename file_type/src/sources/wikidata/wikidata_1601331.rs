use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1601331: FileFormat = FileFormat {
    id: 1_601_331,
    puid: "wikidata/1601331",
    name: "Transport Neutral Encapsulation Format",
    extensions: &["dat", "tnef"],
    media_types: &["application/vnd.ms-tnef", "application/vnd.ms-tnef"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x9F, 0x3E, 0x22])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x9F, 0x3E, 0x22])],
                },
            }],
        },
    ],
    related_formats: &[],
};
