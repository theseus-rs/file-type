use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850741: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_741,
        source_type: SourceType::Wikidata,
        name: "RAR registration data",
        extensions: &["key"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x41, 0x52, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
                        0x74, 0x69, 0x6F, 0x6E, 0x20, 0x64, 0x61, 0x74, 0x61,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
