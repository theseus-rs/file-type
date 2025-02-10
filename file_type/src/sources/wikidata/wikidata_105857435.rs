use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857435: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_435,
        source_type: SourceType::Wikidata,
        name: "Japanese Word Processor Kanji Font",
        extensions: &["f00"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x37, 0x3F, 0x4E, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
