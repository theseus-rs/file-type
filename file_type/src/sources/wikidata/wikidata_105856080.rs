use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856080: FileFormat = FileFormat {
    id: 105_856_080,
    puid: "wikidata/105856080",
    name: "CHEMVIEW animation Data",
    extensions: &["d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x61, 0x74, 0x6F, 0x6D, 0x6C, 0x6F, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x28,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
