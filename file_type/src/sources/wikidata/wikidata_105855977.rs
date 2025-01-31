use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855977: FileFormat = FileFormat {
    id: 105_855_977,
    puid: "wikidata/105855977",
    name: "COMit modems configuration",
    extensions: &["dat", "dos"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x44, 0x45, 0x4D, 0x3D])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x44, 0x45, 0x4D, 0x3D])],
                },
            }],
        },
    ],
    related_formats: &[],
};
