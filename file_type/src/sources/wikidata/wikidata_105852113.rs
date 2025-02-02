use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852113: FileFormat = FileFormat {
    id: 105_852_113,
    source_type: SourceType::Wikidata,
    name: "Structured Titles subtitles (UTF-8)",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x64,
                    0x20, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
