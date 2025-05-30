use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861729: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_729,
        source_type: SourceType::Wikidata,
        name: "ANIMagic DVE Map",
        extensions: &["map"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x56, 0x45, 0x31, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
