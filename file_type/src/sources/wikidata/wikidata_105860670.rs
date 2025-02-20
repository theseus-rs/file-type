use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860670: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_670,
        source_type: SourceType::Wikidata,
        name: "Reason Remote Mapping (UTF-8)",
        extensions: &["remotemap"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x50, 0x72, 0x6F, 0x70, 0x65, 0x6C, 0x6C, 0x65, 0x72,
                        0x68, 0x65, 0x61, 0x64, 0x20, 0x52, 0x65, 0x6D, 0x6F, 0x74, 0x65, 0x20,
                        0x4D, 0x61, 0x70, 0x70, 0x69, 0x6E, 0x67, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
