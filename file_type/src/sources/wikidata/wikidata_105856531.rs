use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856531: FileFormat = FileFormat {
    id: 105_856_531,
    puid: "wikidata/105856531",
    name: "WIBU-SYSTEMS Control",
    extensions: &["wibucmrau"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x57, 0x49, 0x42, 0x55, 0x2D, 0x53, 0x59, 0x53, 0x54, 0x45, 0x4D, 0x53,
                    0x20, 0x43, 0x6F, 0x6E, 0x74, 0x72, 0x6F, 0x6C, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
