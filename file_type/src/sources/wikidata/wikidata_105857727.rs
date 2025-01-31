use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857727: FileFormat = FileFormat {
    id: 105_857_727,
    puid: "wikidata/105857727",
    name: "CBASIC Intermediate code",
    extensions: &["int"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x31, 0x24, 0x2A, 0x00, 0x08, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
