use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862213: FileFormat = FileFormat {
    id: 105_862_213,
    source_type: SourceType::Wikidata,
    name: "Maple compressed Worksheet",
    extensions: &["mwz"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x63, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x77, 0x6F,
                    0x72, 0x6B, 0x73, 0x68, 0x65, 0x65, 0x74, 0x20, 0x6D, 0x61, 0x6A, 0x6F, 0x72,
                    0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
