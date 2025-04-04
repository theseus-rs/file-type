use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849784: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_784,
        source_type: SourceType::Wikidata,
        name: "CWTool disk image (text)",
        extensions: &["cwt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x63, 0x77, 0x74, 0x6F, 0x6F, 0x6C, 0x20, 0x72, 0x61, 0x77,
                        0x20, 0x74, 0x65, 0x78, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
