use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858915: FileFormat = FileFormat {
    id: 105_858_915,
    puid: "wikidata/105858915",
    name: "VDC BitMap (v3)",
    extensions: &["bm", "vbm"],
    media_types: &["image/x-commodore-vbm", "image/x-commodore-vbm"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4D, 0xCB, 0x03])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4D, 0xCB, 0x03])],
                },
            }],
        },
    ],
    related_formats: &[],
};
