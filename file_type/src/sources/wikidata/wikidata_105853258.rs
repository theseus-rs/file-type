use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853258: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_258,
        source_type: SourceType::Wikidata,
        name: "Start++ script",
        extensions: &["startlet"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x3C, 0x53, 0x74, 0x61, 0x72, 0x74, 0x6C, 0x65, 0x74,
                        0x20, 0x41, 0x75, 0x74, 0x68, 0x6F, 0x72, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
