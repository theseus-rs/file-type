use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28600734: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_734,
        source_type: SourceType::Wikidata,
        name: "ESRI Arc/Info Export File",
        extensions: &["X00", "e00", "x00"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x58, 0x50, 0x20, 0x20, 0x30, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
