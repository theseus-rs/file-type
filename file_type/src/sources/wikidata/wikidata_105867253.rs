use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867253: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_253,
        source_type: SourceType::Wikidata,
        name: "Nikon Custom Picture Control",
        extensions: &["ncp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x43, 0x50, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x24,
                        0x30, 0x31, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
