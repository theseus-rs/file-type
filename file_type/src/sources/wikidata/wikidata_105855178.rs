use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855178: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_178,
        source_type: SourceType::Wikidata,
        name: "VisualBasic Form (v6.0)",
        extensions: &["frm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x35, 0x2E, 0x30, 0x30,
                        0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
