use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3359832: FileFormat = FileFormat {
    id: 3_359_832,
    puid: "wikidata/3359832",
    name: "Product Representation Compact",
    extensions: &["prc", "prc"],
    media_types: &["application/octet-stream", "model/prc"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x52, 0x43])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x52, 0x43])],
                },
            }],
        },
    ],
    related_formats: &[],
};
