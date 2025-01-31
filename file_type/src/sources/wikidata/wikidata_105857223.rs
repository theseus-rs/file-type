use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857223: FileFormat = FileFormat {
    id: 105_857_223,
    puid: "wikidata/105857223",
    name: "SymbOS Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x59, 0x4D, 0x48, 0x4C, 0x50, 0x31, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
