use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849667: FileFormat = FileFormat {
    id: 105_849_667,
    puid: "wikidata/105849667",
    name: "Windows Clipboard (Win3.1)",
    extensions: &["clp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0xC3])],
            },
        }],
    }],
    related_formats: &[],
};
