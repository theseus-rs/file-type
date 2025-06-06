use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851629: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_629,
        source_type: SourceType::Wikidata,
        name: "Text - UTF-16 (BE) encoded",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
