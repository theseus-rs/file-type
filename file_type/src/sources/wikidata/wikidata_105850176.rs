use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850176: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_176,
        source_type: SourceType::Wikidata,
        name: "Gaussian Cube data",
        extensions: &["cube"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x20, 0x43, 0x50, 0x4D, 0x44, 0x20, 0x43, 0x55, 0x42, 0x45, 0x20, 0x46,
                        0x49, 0x4C, 0x45, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
