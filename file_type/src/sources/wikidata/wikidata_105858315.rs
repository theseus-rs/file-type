use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858315: FileFormat = FileFormat {
    id: 105_858_315,
    puid: "wikidata/105858315",
    name: "BitCom Emulation settings",
    extensions: &["emu"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x4E, 0x49, 0x54, 0x49, 0x41, 0x4C, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
