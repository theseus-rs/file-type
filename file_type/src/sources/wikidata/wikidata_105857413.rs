use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857413: FileFormat = FileFormat {
    id: 105_857_413,
    puid: "wikidata/105857413",
    name: "JTAG Indirect Configuration",
    extensions: &["jic"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x49, 0x43, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
