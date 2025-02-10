use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856383: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_383,
        source_type: SourceType::Wikidata,
        name: "Adobe Dimensions geometry data (v3.0)",
        extensions: &["dim"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x21, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x44, 0x69, 0x6D, 0x65,
                        0x6E, 0x73, 0x69, 0x6F, 0x6E, 0x73, 0x20, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
