use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762916: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_916,
        source_type: SourceType::Wikidata,
        name: "Logger Pro data",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x3E, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
