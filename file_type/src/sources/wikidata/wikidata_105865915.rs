use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865915: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_915,
        source_type: SourceType::Wikidata,
        name: "PPrint Pattern",
        extensions: &["pat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x54, 0x50, 0x2E, 0x50, 0x41, 0x54, 0x54, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
