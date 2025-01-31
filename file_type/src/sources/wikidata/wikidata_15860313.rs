use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_15860313: FileFormat = FileFormat {
    id: 15_860_313,
    puid: "wikidata/15860313",
    name: "Mathematica Notebook",
    extensions: &["nb", "nb"],
    media_types: &[
        "application/mathematica",
        "application/vnd.wolfram.mathematica",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x28, 0x2A])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x28, 0x2A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
