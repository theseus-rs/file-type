use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5381415: FileFormat = FileFormat {
    id: 5_381_415,
    puid: "wikidata/5381415",
    name: "Envoy",
    extensions: &["evy", "evy"],
    media_types: &["application/x-envoy", "application/x-envoy"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x5E, 0x10, 0x10])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB2, 0x97, 0xE1, 0x69])],
                },
            }],
        },
    ],
    related_formats: &[],
};
