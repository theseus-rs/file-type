use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859920: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_920,
        source_type: SourceType::Wikidata,
        name: "SER format video",
        extensions: &["ser"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x55, 0x43, 0x41, 0x4D, 0x2D, 0x52, 0x45, 0x43, 0x4F, 0x52, 0x44,
                        0x45, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
