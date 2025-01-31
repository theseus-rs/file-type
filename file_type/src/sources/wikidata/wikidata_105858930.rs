use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858930: FileFormat = FileFormat {
    id: 105_858_930,
    puid: "wikidata/105858930",
    name: "VDC BitMap (v2)",
    extensions: &["bm", "vbm"],
    media_types: &["image/x-commodore-vbm", "image/x-commodore-vbm"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4D, 0xCB, 0x02])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4D, 0xCB, 0x02])],
                },
            }],
        },
    ],
    related_formats: &[],
};
