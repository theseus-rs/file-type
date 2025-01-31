use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861232: FileFormat = FileFormat {
    id: 105_861_232,
    puid: "wikidata/105861232",
    name: "InstallShield Language Identifier",
    extensions: &["lid"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4C, 0x61, 0x6E, 0x67, 0x75, 0x61, 0x67, 0x65, 0x73, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
