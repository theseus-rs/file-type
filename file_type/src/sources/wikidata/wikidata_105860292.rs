use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860292: FileFormat = FileFormat {
    id: 105_860_292,
    source_type: SourceType::Wikidata,
    name: "X-Stitch Designer Gold Template",
    extensions: &["rxt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x03, 0x00, 0x00, 0x00, 0x58, 0x2D, 0x53, 0x74, 0x69, 0x74, 0x63, 0x68, 0x20,
                    0x44, 0x65, 0x73, 0x69, 0x67, 0x6E, 0x65, 0x72, 0x20, 0x47, 0x6F, 0x6C, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
