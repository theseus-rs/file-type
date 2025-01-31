use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858494: FileFormat = FileFormat {
    id: 105_858_494,
    puid: "wikidata/105858494",
    name: "Xilinx Bitstream",
    extensions: &["bit"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x09, 0x0F, 0xF0, 0x0F, 0xF0, 0x0F, 0xF0, 0x0F, 0xF0, 0x00, 0x00, 0x01,
                    0x61, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
