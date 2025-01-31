use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857260: FileFormat = FileFormat {
    id: 105_857_260,
    puid: "wikidata/105857260",
    name: "HP Raster Transfer Language",
    extensions: &["rtl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x1B, 0x45, 0x1B, 0x25, 0x30, 0x42, 0x42, 0x50, 0x31, 0x2C, 0x22, 0x62, 0x69,
                    0x74, 0x6D, 0x61, 0x70,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
