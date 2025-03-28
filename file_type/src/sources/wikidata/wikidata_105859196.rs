use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859196: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_196,
        source_type: SourceType::Wikidata,
        name: "Quilt block (v8)",
        extensions: &["blk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x51, 0x55, 0x4C, 0x54, 0x56, 0x45, 0x52, 0x38, 0x54, 0x59, 0x03, 0x00,
                        0x42, 0x4C, 0x4B, 0x4E, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
