use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858297: FileFormat = FileFormat {
    id: 105_858_297,
    source_type: SourceType::Wikidata,
    name: "EnVision Publisher DTP document",
    extensions: &["evp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x6E, 0x56, 0x69, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x50, 0x75, 0x62, 0x6C,
                    0x69, 0x73, 0x68, 0x65, 0x72, 0x20, 0x44, 0x54, 0x50, 0x20, 0x64, 0x6F, 0x63,
                    0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x2E, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
