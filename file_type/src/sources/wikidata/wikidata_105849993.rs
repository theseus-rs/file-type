use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849993: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_993,
        source_type: SourceType::Wikidata,
        name: "CONTEC Logger Binary data",
        extensions: &["clb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4F, 0x4E, 0x54, 0x45, 0x43, 0x20, 0x44, 0x41, 0x54, 0x41, 0x20,
                        0x4C, 0x4F, 0x47, 0x47, 0x45, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
