use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3256475: FileFormat = FileFormat {
    id: 3_256_475,
    puid: "wikidata/3256475",
    name: "Tiny Tafel",
    extensions: &["tt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
