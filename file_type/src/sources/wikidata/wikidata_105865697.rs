use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865697: FileFormat = FileFormat {
    id: 105_865_697,
    puid: "wikidata/105865697",
    name: "Messenger Plus! Encrypted chat log",
    extensions: &["ple"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x10, 0x01, 0x4D, 0x50, 0x4C, 0x45, 0x31, 0x3C, 0x3C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
