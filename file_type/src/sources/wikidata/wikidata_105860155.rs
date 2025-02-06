use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860155: FileFormat = FileFormat {
    id: 105_860_155,
    source_type: SourceType::Wikidata,
    name: "RStudio Project",
    extensions: &["rproj"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20, 0x31, 0x2E, 0x30, 0x0A,
                    0x0A, 0x52, 0x65, 0x73, 0x74, 0x6F, 0x72, 0x65, 0x57, 0x6F, 0x72, 0x6B, 0x73,
                    0x70, 0x61, 0x63, 0x65, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
