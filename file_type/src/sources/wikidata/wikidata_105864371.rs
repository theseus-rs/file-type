use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864371: FileFormat = FileFormat {
    id: 105_864_371,
    puid: "wikidata/105864371",
    name: "Formatta Portable Form File",
    extensions: &["pff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x46, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
