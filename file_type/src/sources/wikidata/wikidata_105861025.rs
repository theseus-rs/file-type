use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861025: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_025,
        source_type: SourceType::Wikidata,
        name: "WinWay Letter (v4.0)",
        extensions: &["ltr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0E, 0x00, 0x01, 0x00, 0x26, 0x00, 0x00, 0x00, 0x40, 0x40, 0x00, 0x00,
                        0x40, 0x40, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
                        0x09, 0x00, 0x08, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
