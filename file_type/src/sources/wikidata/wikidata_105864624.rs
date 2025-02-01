use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864624: FileFormat = FileFormat {
    id: 105_864_624,
    puid: "wikidata/105864624",
    name: "IBM Personal Science Laboratory experiment",
    extensions: &["psl", "pss"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x45, 0x52, 0x43])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x45, 0x52, 0x43])],
                },
            }],
        },
    ],
    related_formats: &[],
};
