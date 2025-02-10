use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855287: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_287,
        source_type: SourceType::Wikidata,
        name: "File Investigator data (generic)",
        extensions: &["fib", "fid", "fih", "fip", "fiv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x69, 0x6C, 0x65, 0x20, 0x49, 0x6E, 0x76, 0x65, 0x73, 0x74, 0x69,
                        0x67, 0x61, 0x74, 0x6F, 0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
