use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855423: FileFormat = FileFormat {
    id: 105_855_423,
    puid: "wikidata/105855423",
    name: "AVG Antivirus Vault file",
    extensions: &["fil"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x56, 0x47, 0x5F, 0x56, 0x41, 0x55, 0x4C, 0x54, 0xCD,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
