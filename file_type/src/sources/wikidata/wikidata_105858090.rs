use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858090: FileFormat = FileFormat {
    id: 105_858_090,
    puid: "wikidata/105858090",
    name: "MAME Input (Versioned Header)",
    extensions: &["inp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x4D, 0x45, 0x49, 0x4E, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
