use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_7618055: FileType = FileType {
    file_format: &FileFormat {
        id: 7_618_055,
        source_type: SourceType::Wikidata,
        name: "Stockholm format",
        extensions: &["stk", "sto"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x53, 0x54, 0x4F, 0x43, 0x4B, 0x48, 0x4F, 0x4C, 0x4D, 0x20,
                        0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
