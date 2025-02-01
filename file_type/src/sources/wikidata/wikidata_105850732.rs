use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850732: FileFormat = FileFormat {
    id: 105_850_732,
    puid: "wikidata/105850732",
    name: "Java KeyStore",
    extensions: &["jks", "keystore"],
    media_types: &["application/x-java-keystore", "application/x-java-keystore"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xED, 0xFE, 0xED])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xED, 0xFE, 0xED])],
                },
            }],
        },
    ],
    related_formats: &[],
};
