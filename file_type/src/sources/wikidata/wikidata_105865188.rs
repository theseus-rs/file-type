use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865188: FileFormat = FileFormat {
    id: 105_865_188,
    puid: "wikidata/105865188",
    name: "PowerWindows Project (v2.0)",
    extensions: &["pw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6F, 0x77, 0x65, 0x72, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x20,
                    0x76, 0x32, 0x2E, 0x30, 0x20, 0xA9, 0x31, 0x39, 0x38, 0x37, 0x20, 0x62, 0x79,
                    0x20, 0x49, 0x4E, 0x4F, 0x56, 0x41, 0x54, 0x52, 0x4F, 0x4E, 0x49, 0x43, 0x53,
                    0x2C, 0x20, 0x49, 0x4E, 0x43, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
