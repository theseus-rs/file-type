use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860331: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_331,
        source_type: SourceType::Wikidata,
        name: "Xilinx ASCII Bitstream",
        extensions: &["rbt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x69, 0x6C, 0x69, 0x6E, 0x78, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49,
                        0x20, 0x42, 0x69, 0x74, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
