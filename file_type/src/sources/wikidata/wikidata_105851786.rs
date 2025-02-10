use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851786: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_786,
        source_type: SourceType::Wikidata,
        name: "SETI@Home results",
        extensions: &["sah"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x72, 0x65, 0x73, 0x75, 0x6C, 0x74, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
