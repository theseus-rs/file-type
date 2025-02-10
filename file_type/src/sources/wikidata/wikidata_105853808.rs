use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853808: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_808,
        source_type: SourceType::Wikidata,
        name: "AutoIt v3 compiled script",
        extensions: &["a3x"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xA3, 0x48, 0x4B, 0xBE, 0x98, 0x6C, 0x4A, 0xA9, 0x99, 0x4C, 0x53, 0x0A,
                        0x86, 0xD6, 0x48, 0x7D, 0x41, 0x55, 0x33, 0x21, 0x45, 0x41, 0x30, 0x36,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
