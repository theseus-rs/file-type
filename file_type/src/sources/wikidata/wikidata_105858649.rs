use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858649: FileFormat = FileFormat {
    id: 105_858_649,
    puid: "wikidata/105858649",
    name: "GoDEX character translation table",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x6F, 0x64, 0x65, 0x78, 0x2C, 0x56, 0x30, 0x30, 0x30, 0x2C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
