use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762792: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_792,
        source_type: SourceType::Wikidata,
        name: "Xilinx Board Description format",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x54, 0x54, 0x52, 0x49, 0x42, 0x55, 0x54, 0x45, 0x20, 0x56, 0x45,
                        0x4E, 0x44, 0x4F, 0x52, 0x20, 0x3D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
