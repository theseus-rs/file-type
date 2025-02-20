use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72727969: FileType = FileType {
    file_format: &FileFormat {
        id: 72_727_969,
        source_type: SourceType::Wikidata,
        name: "Windows for Pen Computing Notebook",
        extensions: &["ntb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xCA, 0xFE, 0x20, 0x00, 0x00, 0x02, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
