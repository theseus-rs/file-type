use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865336: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_336,
        source_type: SourceType::Wikidata,
        name: "PC-Axis data (var 1)",
        extensions: &["px"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
