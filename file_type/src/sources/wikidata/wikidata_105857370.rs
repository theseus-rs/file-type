use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857370: FileFormat = FileFormat {
    id: 105_857_370,
    puid: "wikidata/105857370",
    name: "JasperReports Print",
    extensions: &["jrprint"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xAC, 0xED, 0x00, 0x05, 0x73, 0x72, 0x00, 0x27, 0x6E, 0x65, 0x74, 0x2E, 0x73,
                    0x66, 0x2E, 0x6A, 0x61, 0x73, 0x70, 0x65, 0x72, 0x72, 0x65, 0x70, 0x6F, 0x72,
                    0x74, 0x73, 0x2E, 0x65, 0x6E, 0x67, 0x69, 0x6E, 0x65, 0x2E, 0x4A, 0x61, 0x73,
                    0x70, 0x65, 0x72, 0x50, 0x72, 0x69, 0x6E, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
