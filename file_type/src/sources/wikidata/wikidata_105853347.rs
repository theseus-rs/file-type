use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853347: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_347,
        source_type: SourceType::Wikidata,
        name: "SEC EDGAR document",
        extensions: &["txt"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x53, 0x45, 0x43, 0x2D, 0x44, 0x4F, 0x43, 0x55, 0x4D, 0x45, 0x4E,
                        0x54, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
