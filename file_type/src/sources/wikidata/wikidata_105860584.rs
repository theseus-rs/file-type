use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860584: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_584,
        source_type: SourceType::Wikidata,
        name: "Raw Bitmap Font",
        extensions: &["rbf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xE0, 0x0E, 0xF0, 0x0D, 0x03, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
