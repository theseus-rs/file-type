use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856186: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_186,
        source_type: SourceType::Wikidata,
        name: "Personal Font Maker Definitions (charset)",
        extensions: &["def"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x44, 0x4D, 0x20, 0x44, 0x45, 0x46, 0x53, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
