use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853203: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_203,
        source_type: SourceType::Wikidata,
        name: "Swift Interchange File V2 (UTF-8)",
        extensions: &["sif"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x23, 0x20, 0x53, 0x57, 0x49, 0x46, 0x54, 0x20, 0x49,
                        0x4E, 0x54, 0x45, 0x52, 0x43, 0x48, 0x41, 0x4E, 0x47, 0x45, 0x20, 0x46,
                        0x49, 0x4C, 0x45, 0x20, 0x56, 0x32, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
