use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853574: FileFormat = FileFormat {
    id: 105_853_574,
    puid: "wikidata/105853574",
    name: "ZX-Live Snapshot",
    extensions: &["zls"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x58, 0x2D, 0x4C, 0x69, 0x76, 0x65, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
