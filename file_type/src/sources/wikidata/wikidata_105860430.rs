use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860430: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_430,
        source_type: SourceType::Wikidata,
        name: "Borland Reflex Database",
        extensions: &["rxd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x02, 0x33, 0x51, 0x2E, 0x21, 0x26, 0x40, 0x23, 0x24, 0x21, 0x26,
                        0x26,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
