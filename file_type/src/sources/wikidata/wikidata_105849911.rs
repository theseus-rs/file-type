use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849911: FileFormat = FileFormat {
    id: 105_849_911,
    puid: "wikidata/105849911",
    name: "WinWorks Chart",
    extensions: &["cht"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x69, 0x6E, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x20, 0x43, 0x68, 0x61, 0x72,
                    0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
