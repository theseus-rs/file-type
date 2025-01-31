use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857377: FileFormat = FileFormat {
    id: 105_857_377,
    puid: "wikidata/105857377",
    name: "Trizbort.io map",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x22, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73, 0x22, 0x3A, 0x7B,
                    0x22, 0x67, 0x72, 0x69, 0x64, 0x22, 0x3A, 0x7B, 0x22, 0x76, 0x69, 0x73, 0x69,
                    0x62, 0x6C, 0x65, 0x22, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
