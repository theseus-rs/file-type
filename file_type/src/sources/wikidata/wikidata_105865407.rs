use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865407: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_407,
        source_type: SourceType::Wikidata,
        name: "PC-Type Printer escape codes",
        extensions: &["prn"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x2D, 0x54, 0x59, 0x50, 0x45, 0x20, 0x45, 0x73, 0x63, 0x61,
                        0x70, 0x65, 0x20, 0x63, 0x6F, 0x64, 0x65, 0x73, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
