use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858488: FileFormat = FileFormat {
    id: 105_858_488,
    puid: "wikidata/105858488",
    name: "Tiny Stuff format bitmap (med-res anim)",
    extensions: &["tn5", "tny"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x04, 0x07, 0x77])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x04, 0x07, 0x77])],
                },
            }],
        },
    ],
    related_formats: &[],
};
