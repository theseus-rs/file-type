use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856450: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_450,
        source_type: SourceType::Wikidata,
        name: "Wireless Markup Language file",
        extensions: &["wml"],
        media_types: &["text/vnd.wap.wml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
