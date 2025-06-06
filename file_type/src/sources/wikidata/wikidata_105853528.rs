use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853528: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_528,
        source_type: SourceType::Wikidata,
        name: "ZAP Patch",
        extensions: &["zap"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x34, 0x32, 0x36, 0x20, 0x30, 0x37, 0x31, 0x20, 0x30, 0x34, 0x37, 0x20,
                        0x35, 0x30, 0x31, 0x20, 0x31, 0x37, 0x34, 0x20, 0x33, 0x32, 0x31, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
