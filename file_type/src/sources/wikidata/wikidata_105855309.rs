use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855309: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_309,
        source_type: SourceType::Wikidata,
        name: "VisualBasic Form (v3.x)",
        extensions: &["frm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x32, 0x2E, 0x30, 0x30,
                        0x0D, 0x0A, 0x42, 0x65, 0x67, 0x69, 0x6E, 0x20, 0x46, 0x6F, 0x72, 0x6D,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
