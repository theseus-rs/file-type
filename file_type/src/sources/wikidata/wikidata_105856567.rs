use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856567: FileFormat = FileFormat {
    id: 105_856_567,
    puid: "wikidata/105856567",
    name: "Applause Word data",
    extensions: &["w"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x6F, 0x72, 0x64, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
