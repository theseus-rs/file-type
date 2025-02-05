use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857251: FileFormat = FileFormat {
    id: 105_857_251,
    source_type: SourceType::Wikidata,
    name: "Free Hex Editor Neo layout",
    extensions: &["hexdwc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x1A, 0xE1, 0x85, 0xD3, 0x6E, 0x32, 0x4E, 0xAB, 0xB9, 0x61, 0x46, 0xE4,
                    0x6B, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
