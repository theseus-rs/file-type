use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853585: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_585,
        source_type: SourceType::Wikidata,
        name: "BlueZone VT Display Emulator configuration",
        extensions: &["zvt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x5A, 0x56, 0x54, 0x31, 0x30, 0x30, 0x41, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
