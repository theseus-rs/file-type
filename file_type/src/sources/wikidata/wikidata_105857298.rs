use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857298: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_298,
        source_type: SourceType::Wikidata,
        name: "HP Printer Command Language (PCL6)",
        extensions: &["prn", "px3", "pxl"],
        media_types: &["application/vnd.hp-PCL"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1B, 0x25, 0x2D, 0x31, 0x32, 0x33, 0x34, 0x35, 0x58, 0x40, 0x50, 0x4A,
                        0x4C, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
