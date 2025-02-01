use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860635: FileFormat = FileFormat {
    id: 105_860_635,
    puid: "wikidata/105860635",
    name: "Klasik text Resouce data",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x6C, 0x61, 0x73, 0x69, 0x6B, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x72,
                    0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
