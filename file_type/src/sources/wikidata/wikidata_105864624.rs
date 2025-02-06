use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864624: FileFormat = FileFormat {
    id: 105_864_624,
    source_type: SourceType::Wikidata,
    name: "IBM Personal Science Laboratory experiment",
    extensions: &["psl", "pss"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x45, 0x52, 0x43])],
                },
            }],
        },
        Signature {
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
