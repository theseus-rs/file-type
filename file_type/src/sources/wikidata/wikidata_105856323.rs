use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856323: FileFormat = FileFormat {
    id: 105_856_323,
    puid: "wikidata/105856323",
    name: "Windows 64bit Memory Dump",
    extensions: &["dmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x47, 0x45, 0x44, 0x55, 0x36, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
