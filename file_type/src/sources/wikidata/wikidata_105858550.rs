use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858550: FileFormat = FileFormat {
    id: 105_858_550,
    puid: "wikidata/105858550",
    name: "MediaTek combined download agent package",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x54, 0x4B, 0x5F, 0x44, 0x4F, 0x57, 0x4E, 0x4C, 0x4F, 0x41, 0x44, 0x5F,
                    0x41, 0x47, 0x45, 0x4E, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
