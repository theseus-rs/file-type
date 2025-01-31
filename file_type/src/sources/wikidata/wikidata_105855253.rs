use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855253: FileFormat = FileFormat {
    id: 105_855_253,
    puid: "wikidata/105855253",
    name: "AVG Antivirus Vault file (v7)",
    extensions: &["fil"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x56, 0x47, 0x37, 0x5F, 0x41, 0x4E, 0x54, 0x49, 0x56, 0x49, 0x52, 0x55,
                    0x53, 0x5F, 0x56, 0x41, 0x55, 0x4C, 0x54, 0x5F, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
