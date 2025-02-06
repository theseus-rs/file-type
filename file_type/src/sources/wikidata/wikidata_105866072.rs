use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866072: FileFormat = FileFormat {
    id: 105_866_072,
    source_type: SourceType::Wikidata,
    name: "Pod Specification",
    extensions: &["podspec"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6F, 0x64, 0x3A, 0x3A, 0x53, 0x70, 0x65, 0x63, 0x2E, 0x6E, 0x65, 0x77,
                    0x20, 0x64, 0x6F, 0x20, 0x7C, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
