use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858265: FileFormat = FileFormat {
    id: 105_858_265,
    puid: "wikidata/105858265",
    name: "DCMO5 emulator tape image",
    extensions: &["k7"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x43, 0x4D, 0x4F, 0x35, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                    0x01, 0x01, 0x01, 0x3C, 0x5A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
