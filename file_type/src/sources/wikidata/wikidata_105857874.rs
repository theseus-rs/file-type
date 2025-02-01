use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857874: FileFormat = FileFormat {
    id: 105_857_874,
    puid: "wikidata/105857874",
    name: "Applause II settings (v1.5)",
    extensions: &["ini"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x73, 0x68, 0x74, 0x6F, 0x6E, 0x2D, 0x54, 0x61, 0x74, 0x65, 0x20, 0x41,
                    0x50, 0x50, 0x4C, 0x41, 0x55, 0x53, 0x45, 0x20, 0x49, 0x49, 0x20, 0x76, 0x65,
                    0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x2E, 0x35, 0x20, 0x49, 0x4E, 0x49,
                    0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
