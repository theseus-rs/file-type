use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851230: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_230,
        source_type: SourceType::Wikidata,
        name: "Text - UTF-EBCDIC encoded",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDD, 0x73, 0x66, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
