use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864887: FileFormat = FileFormat {
    id: 105_864_887,
    puid: "wikidata/105864887",
    name: "WinCC script",
    extensions: &["pas"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x50, 0x41, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
