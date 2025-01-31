use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849696: FileFormat = FileFormat {
    id: 105_849_696,
    puid: "wikidata/105849696",
    name: "Security Certificate",
    extensions: &["crt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x61, 0x67, 0x20, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
                    0x73, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
