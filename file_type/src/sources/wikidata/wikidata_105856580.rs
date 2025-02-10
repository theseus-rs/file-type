use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856580: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_580,
        source_type: SourceType::Wikidata,
        name: "Xilinx ISE Simulator Waveform DataBase",
        extensions: &["wdb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0x49, 0xBD, 0x87, 0x40, 0xA6, 0x39, 0x3E, 0xDC, 0x66, 0x23, 0xAA,
                        0x9E, 0xE1, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
