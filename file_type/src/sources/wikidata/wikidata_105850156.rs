use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850156: FileFormat = FileFormat {
    id: 105_850_156,
    puid: "wikidata/105850156",
    name: "Windows Clipboard (NT)",
    extensions: &["clp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0xC3])],
            },
        }],
    }],
    related_formats: &[],
};
