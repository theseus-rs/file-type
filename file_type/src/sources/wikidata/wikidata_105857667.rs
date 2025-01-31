use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857667: FileFormat = FileFormat {
    id: 105_857_667,
    puid: "wikidata/105857667",
    name: "EP128Emu tape image",
    extensions: &["tap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x02, 0x75, 0xCD, 0x72, 0x1C, 0x44, 0x51, 0x26, 0x00, 0x00, 0x00, 0x01, 0x00,
                    0x00, 0x5D, 0xC0,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
