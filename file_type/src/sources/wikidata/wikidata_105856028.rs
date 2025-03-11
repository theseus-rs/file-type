use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856028: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_028,
        source_type: SourceType::Wikidata,
        name: "Document Safer marked",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x44, 0x4F, 0x43, 0x55, 0x4D, 0x45, 0x4E, 0x54, 0x20, 0x53, 0x41,
                        0x46, 0x45, 0x52, 0x20, 0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
