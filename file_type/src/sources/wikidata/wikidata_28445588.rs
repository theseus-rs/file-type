use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445588: FileFormat = FileFormat {
    id: 28_445_588,
    puid: "wikidata/28445588",
    name: "ALZ",
    extensions: &["alz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4C, 0x5A, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
