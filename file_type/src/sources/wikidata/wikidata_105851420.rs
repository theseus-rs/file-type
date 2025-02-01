use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851420: FileFormat = FileFormat {
    id: 105_851_420,
    puid: "wikidata/105851420",
    name: "TR Assist Dictionary",
    extensions: &["trd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x46, 0x49, 0x4C, 0x45, 0x5F, 0x54, 0x59, 0x50, 0x45, 0x3A, 0x44, 0x49,
                    0x43, 0x54, 0x0D, 0x0A, 0x24, 0x44, 0x49, 0x43, 0x54, 0x5F, 0x54, 0x59, 0x50,
                    0x45, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
