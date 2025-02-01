use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860249: FileFormat = FileFormat {
    id: 105_860_249,
    puid: "wikidata/105860249",
    name: "RedTitan Zip",
    extensions: &["rtz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x45, 0x44, 0x54, 0x49, 0x54, 0x41, 0x4E, 0x20, 0x5A, 0x49, 0x50, 0x0D,
                    0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
